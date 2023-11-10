use std::str::FromStr;
use http::method::Method;
use http::request::Request;

/// # Validator
/// Validate results
/// - Validate by created files
/// - Validate by console output
/// - Validate by http response
/// - Validate by http response code



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
    pub result: bool,
    pub response: String,
    pub response_code: u16,
}

impl Validator {
    pub fn new() -> Self {
        Validator {
            files: vec![],
            console: vec![],
            http: vec![],
        }
    }

    pub fn add_file(&mut self, path: String, content: String) {
        self.files.push(File {
            path,
            content,
        });
    }

    pub fn add_console(&mut self, content: String) {
        self.console.push(content);
    }

    pub fn add_http(&mut self, payload: String, url: String, method: HTTPMethod, headers: Vec<String>, response: String, response_code: u16, benchmark: bool) {
        self.http.push(HTTP {
            payload,
            url,
            method,
            headers,
            response,
            response_code,
            benchmark,
        });
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
        let real= out.lines();
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

            let request = Request::builder()
                .uri(&http.url)
                .method(method);

            for header in &http.headers {
                let mut split = header.split(":");
                let key = split.next().unwrap();
                let value = split.next().unwrap();

                request.header(key, value);
            }

            //TODO hmm, the request crate is not what I thought it was...

            let response;



            let result = response == http.response && response_code == http.response_code;

            results.push(HTTPResult {
                http,
                result,
                response,
                response_code,
            });
        }

        results
    }




}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}