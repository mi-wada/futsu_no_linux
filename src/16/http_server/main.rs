use std::fs::File;
use std::io::{stdin, BufReader, Read, Write};
use std::{collections::HashMap, io::BufRead};

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let file_name = std::env::args().nth(1);

    let mut stream = BufReader::new(match file_name {
        Some(file_name) => File::open(file_name)?,
        None => {
            let reader = stdin();

            let mut tempfile = tempfile::tempfile()?;
            for line in reader.lines() {
                let line = line? + "\n";
                tempfile.write_all(line.as_bytes())?;
            }

            tempfile.flush()?;
            tempfile
        }
    });

    let http_request = parse_http_request(&mut stream)?;

    println!("{:?}", http_request);

    Ok(())
}

#[derive(Debug, PartialEq)]
enum HttpMethod {
    GET,
    HEAD,
}

impl TryFrom<&str> for HttpMethod {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "GET" => Ok(HttpMethod::GET),
            "HEAD" => Ok(HttpMethod::HEAD),
            _ => bail!("Invalid HTTP Method"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum HttpVersion {
    V1_1,
    V2,
    V3,
}

impl TryFrom<&str> for HttpVersion {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "HTTP/1.1" => Ok(HttpVersion::V1_1),
            "HTTP/2" => Ok(HttpVersion::V2),
            "HTTP/3" => Ok(HttpVersion::V3),
            _ => bail!("Invalid HTTP Version"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct HttpRequest {
    method: HttpMethod,
    path: String,
    http_version: HttpVersion,
    headers: HashMap<String, String>,
}

#[derive(Default)]
struct ParsingHttpRequest {
    method: Option<HttpMethod>,
    path: Option<String>,
    http_version: Option<HttpVersion>,
    headers: Option<HashMap<String, String>>,
    read_headers: bool,
}

impl ParsingHttpRequest {
    pub fn reading_request_line(&self) -> bool {
        self.method.is_none() && self.path.is_none() && self.http_version.is_none()
    }

    pub fn set_request_line(
        &mut self,
        method: HttpMethod,
        path: String,
        http_version: HttpVersion,
    ) {
        self.method(method);
        self.path(path);
        self.http_version(http_version);
    }

    fn method(&mut self, method: HttpMethod) {
        self.method = Some(method);
    }

    fn path(&mut self, path: String) {
        self.path = Some(path);
    }

    fn http_version(&mut self, http_version: HttpVersion) {
        self.http_version = Some(http_version);
    }

    pub fn reading_headers(&self) -> bool {
        !self.reading_request_line() && !self.read_headers
    }

    pub fn set_header(&mut self, k: String, v: String) {
        match &mut self.headers {
            Some(headers) => {
                headers.insert(k, v);
            }
            None => {
                self.headers = Some(HashMap::new());
                self.set_header(k, v);
            }
        }
    }

    pub fn validate(self) -> Result<HttpRequest> {
        if self.method.is_none()
            || self.path.is_none()
            || self.http_version.is_none()
            || self.headers.is_none()
        {
            bail!("Invalid HTTP Request.")
        } else {
            Ok(HttpRequest {
                method: self.method.unwrap(),
                path: self.path.unwrap(),
                http_version: self.http_version.unwrap(),
                headers: self.headers.unwrap(),
            })
        }
    }
}

fn parse_http_request(stream: &mut impl BufRead) -> Result<HttpRequest> {
    let mut parsing_http_request = ParsingHttpRequest::default();

    for line in stream.lines() {
        let line = line?;

        if parsing_http_request.reading_request_line() {
            let (method, path, http_version) = parse_request_line(&line)?;
            parsing_http_request.set_request_line(method, path, http_version)
        } else if parsing_http_request.reading_headers() {
            if line == "\r\n" {
                parsing_http_request.read_headers = true;
                continue;
            }
            let (key, value) = parse_header_line(&line)?;
            parsing_http_request.set_header(key, value);
        } else {
            // TODO: impl later
        }
    }
    let http_request = parsing_http_request.validate()?;

    Ok(http_request)
}

fn parse_request_line(s: &str) -> Result<(HttpMethod, String, HttpVersion)> {
    let mut splited = s.split(' ');

    let method = HttpMethod::try_from(splited.next().unwrap())?;
    let path = splited.next().unwrap().into();
    let http_version = HttpVersion::try_from(splited.next().unwrap())?;

    Ok((method, path, http_version))
}

fn parse_header_line(s: &str) -> Result<(String, String)> {
    let mut splited = s.split(": ");

    let key = match splited.next() {
        Some(key) => key.into(),
        None => bail!("Header key not found."),
    };

    let value = match splited.next() {
        Some(value) => value.into(),
        None => bail!("Header value not found."),
    };

    Ok((key, value))
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use super::*;

    #[test]
    fn test_parse_http_request() {
        let mut stream =
            BufReader::new(File::open("tests/assets/http_server/valid_request").unwrap());

        let res = parse_http_request(&mut stream).unwrap();
        let expected = {
            let mut headers = HashMap::new();
            headers.insert(
                "User-Agent".into(),
                "Mozilla/4.0 (compatible; MSIE5.01; Windows NT)".into(),
            );
            headers.insert("Host".into(), "example.com".into());
            headers.insert("Connection".into(), "close".into());

            HttpRequest {
                method: HttpMethod::GET,
                path: "/index.html".into(),
                http_version: HttpVersion::V1_1,
                headers,
            }
        };
        assert_eq!(res, expected)
    }

    #[test]
    fn test_request_line() {
        struct TestCase {
            args: &'static str,
            expected: Result<(HttpMethod, String, HttpVersion), anyhow::Error>,
        }

        let test_cases: Vec<TestCase> = vec![TestCase {
            args: "GET /index.html HTTP/1.1",
            expected: Ok((HttpMethod::GET, "/index.html".into(), HttpVersion::V1_1)),
        }];

        for test_case in test_cases {
            let (s, expected) = (test_case.args, test_case.expected);

            match expected {
                Ok(expected) => {
                    assert_eq!(parse_request_line(s).unwrap(), expected);
                }
                Err(_) => {
                    assert!(parse_request_line(s).is_err());
                }
            }
        }
    }

    #[test]
    fn test_parse_header_line() {
        struct TestCase {
            args: &'static str,
            expected: Result<(String, String), anyhow::Error>,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                args: ("Foo-bar: ok"),
                expected: Ok(("Foo-bar".into(), "ok".into())),
            },
            TestCase {
                args: ("Foo-bar:ok"),
                expected: Err(anyhow!("hoge")),
            },
            TestCase {
                args: ("Foo-bar ok"),
                expected: Err(anyhow!("hoge")),
            },
        ];

        for test_case in test_cases {
            let (s, expected) = (test_case.args, test_case.expected);

            match expected {
                Ok(expected) => {
                    assert_eq!(parse_header_line(s).unwrap(), expected);
                }
                Err(_) => {
                    assert!(parse_header_line(s).is_err());
                }
            }
        }
    }
}
