use std::str::FromStr;
use std::thread;
use std::time::Duration;

use reqwest::{Method, StatusCode};
use reqwest::blocking::RequestBuilder as BlockingRequestBuilder;
use reqwest::header::{HeaderMap, HeaderName};

use crate::benchmark::benchmark;

/// # Validator
/// Validate results
/// - Validate by created files
/// - Validate by console output
/// - Validate by http response
/// - Validate by http response code

const BENCHMARK_CONNECTIONS: u16 = 8192;


pub struct Validator {
    pub files: Vec<File>,
    pub console: Vec<String>,
    pub http: Vec<HTTP>,

}

pub struct File {
    pub path: String,
    pub content: String,
}

pub struct HTTP {
    pub payload: String,
    pub url: String,
    pub method: HTTPMethod,
    pub headers: Vec<String>,

    pub response: String,
    pub response_code: u16,

    pub benchmark: bool,
}

pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

pub struct FileResult<'a> {
    pub file: &'a File,
    pub output: String,
    pub result: bool,
}

pub struct ConsoleResult<'a> {
    pub console: &'a Vec<String>,
    pub output: Vec<String>,
    pub result: bool,
    pub res_individually: Vec<bool>,
}

pub struct HTTPResult<'a> {
    pub http: &'a HTTP,
    pub result: HTTPResultType,
    pub response: String,
    pub response_code: u16,
}

pub enum HTTPResultType {
    Success,
    Partial,
    Always,
    Fail,
}

impl Validator {
    pub fn new() -> Self {
        Validator {
            files: vec![],
            console: vec![],
            http: vec![],
        }
    }

    pub fn validate_files(&self) -> Vec<FileResult> {
        let mut results = vec![];

        for file in &self.files {
            let output = std::fs::read_to_string(&file.path).unwrap();
            let result = output == file.content;

            results.push(FileResult {
                file,
                output,
                result,
            });
        }

        let mut success = true;

        for result in &results {
            if !result.result {
                success = false;
                println!("File {} failed", result.file.path);
            }
        }

        if success {
            println!("All files passed");
        }

        results
    }

    pub fn validate_console(&self, out: String) -> ConsoleResult {
        let real = out.lines();
        let expected = self.console.iter();

        let res_individually = real.zip(expected).map(|(r, e)| r == e).collect::<Vec<bool>>();
        let res_individually = res_individually.to_owned();


        let real: Vec<String> = out.lines().map(|s| s.to_string()).collect();
        let expected = &self.console;
        let result = &real == expected;

        ConsoleResult {
            console: &self.console,
            output: real,
            result,
            res_individually,
        }
    }


    pub fn validate_http(&self) -> Vec<HTTPResult> {
        let mut results = vec![];

        for http in &self.http {
            let method = match http.method {
                HTTPMethod::GET => Method::GET,
                HTTPMethod::POST => Method::POST,
                HTTPMethod::PUT => Method::PUT,
                HTTPMethod::DELETE => Method::DELETE,
                HTTPMethod::PATCH => Method::PATCH,
            };


            let mut headers = HeaderMap::new();
            for header in &http.headers {
                let mut split = header.split(':');
                let key = HeaderName::from_str(split.next().unwrap()).unwrap();
                let value = split.next().to_owned().unwrap().parse().unwrap();

                headers.insert(key, value);
            }
            if http.benchmark {
                let client = reqwest::Client::new();

                let request = client
                    .request(method, &http.url)
                    .headers(headers)
                    .body(http.payload.clone());


                let res = benchmark(request, BENCHMARK_CONNECTIONS, Duration::from_secs(5));

                for (success, code, text) in res.1 {
                    results.push(HTTPResult {
                        http,
                        result: if success { HTTPResultType::Success } else { HTTPResultType::Fail },
                        response_code: code,
                        response: text,
                    });
                }
            } else {
                let client = reqwest::blocking::Client::new();


                let request = client
                    .request(method, &http.url)
                    .headers(headers)
                    .body(http.payload.clone());


                let res = check(request, http.response.clone(), http.response_code);

                results.push(HTTPResult {
                    http,
                    result: if res.0 { HTTPResultType::Success } else { HTTPResultType::Fail },
                    response_code: res.1.as_u16(),
                    response: res.2,
                });
            }
        }

        results
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}


fn check(request: BlockingRequestBuilder, response: String, response_code: u16) -> (bool, StatusCode, String) {
    let handle = thread::spawn(|| {
        let res = request.send().unwrap();

        let code = res.status();
        let text = res.text().unwrap();

        (code, text)
    });

    let (code, text) = handle.join().unwrap();

    (code == response_code && text == response, code, text)
}