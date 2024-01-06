use std::fs::File;
use std::io::{stdin, BufReader, Read, Write};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::os::fd::FromRawFd;
use std::str::FromStr;
use std::{collections::HashMap, io::BufRead};

use anyhow::{bail, Result};
use libc::ENAMETOOLONG;

fn main() -> Result<()> {
    let socket_fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    if socket_fd < 0 {
        panic!("Failed to create socket");
    }

    let fd = {
        let addr = libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: 8088_u16.to_be(),
            sin_addr: libc::in_addr {
                s_addr: libc::INADDR_ANY,
            },
            sin_zero: [0; 8],
            sin_len: std::mem::size_of::<libc::sockaddr_in>() as u8,
        };
        if unsafe {
            libc::bind(
                socket_fd,
                &addr as *const _ as *const libc::sockaddr,
                std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
            )
        } < 0
        {
            unsafe {
                libc::close(socket_fd);
            }
            println!(
                "error_code: {}",
                std::io::Error::last_os_error().raw_os_error().unwrap()
            );
            panic!("Failed to bind");
        }

        if unsafe { libc::listen(socket_fd, 5) } < 0 {
            unsafe { libc::close(socket_fd) };
            panic!("Failed to listen");
        }

        let mut client_addr = libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: 0,
            sin_addr: libc::in_addr {
                s_addr: libc::INADDR_ANY,
            },
            sin_zero: [0; 8],
            sin_len: std::mem::size_of::<libc::sockaddr_in>() as u8,
        };
        let mut client_addr_size = std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;

        let client_fd = unsafe {
            libc::accept(
                socket_fd,
                &mut client_addr as *mut _ as *mut libc::sockaddr,
                &mut client_addr_size as *mut libc::socklen_t,
            )
        };
        if client_fd < 0 {
            unsafe { libc::close(socket_fd) };
            panic!("Failed to accept connection");
        }

        client_fd
    };
    let mut stream = { BufReader::new(unsafe { std::fs::File::from_raw_fd(fd) }) };

    let http_request = parse_http_request(&mut stream)?;

    println!("{:?}", http_request);

    let body = BufReader::new(File::open(format!("src/16/www{}", http_request.path))?);
    let mut response = HttpResponse {
        http_version: http_request.http_version,
        content_length: 56,
        content_type: ContentType::HTML,
        connection: "close".into(),
        body: Box::new(body),
    };
    let bytes = response.to_bytes();
    unsafe { libc::write(fd, bytes.as_ptr() as _, bytes.len()) };

    unsafe {
        libc::close(socket_fd);
        libc::close(fd);
    };

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

impl FromStr for HttpVersion {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "HTTP/1.1" => Ok(HttpVersion::V1_1),
            "HTTP/2" => Ok(HttpVersion::V2),
            "HTTP/3" => Ok(HttpVersion::V3),
            _ => bail!("Invalid HTTP Version"),
        }
    }
}

impl HttpVersion {
    fn to_string(&self) -> String {
        match self {
            Self::V1_1 => "HTTP/1.1".into(),
            Self::V2 => "HTTP/2".into(),
            Self::V3 => "HTTP/3".into(),
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
            println!("line: {}", line);
            if line.is_empty() {
                parsing_http_request.read_headers = true;
                break;
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
    let http_version = HttpVersion::from_str(splited.next().unwrap())?;

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

enum ContentType {
    HTML,
    JSON,
}

impl Into<&str> for ContentType {
    fn into(self) -> &'static str {
        match self {
            Self::HTML => "text/html",
            Self::JSON => "application/json",
        }
    }
}

struct HttpResponse {
    http_version: HttpVersion,
    content_length: usize,
    content_type: ContentType,
    connection: String,
    body: Box<dyn BufRead>,
}

impl HttpResponse {
    fn to_bytes(&mut self) -> Vec<u8> {
        let body = {
            let mut body = vec![];
            self.body.read_to_end(&mut body);
            body
        };

        let mut res = (format!("{} 200 OK\r\n", self.http_version.to_string())
            + &format!("Content-length: {}\r\n", body.len())
            + &format!("Content-Type: {}\r\n", "text/html")
            + &format!("Connection: {}\r\n", "close")
            + &format!("\r\n"))
            .into_bytes();
        res.extend(body);

        res
    }
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
