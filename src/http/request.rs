use crate::utils;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

lazy_static! {
    static ref QUERY_PATTERN: Regex = Regex::new(r"\?.*").unwrap();
    static ref BODY_PATTERN: Regex =
        Regex::new(r"^.+ .+ .+\r\n\r\n(?:.+: .+\r\n)*([\s\S]*?)$").unwrap();
}

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Connect,
    Options,
    Trace,
    Unknown,
}

pub struct Request {
    pub headers: HashMap<String, String>,
    pub method: Method,
    pub body: String,
    pub route: String,
    pub query: HashMap<String, String>,
}

pub fn parse(mut stream: &TcpStream) -> Request {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let raw_request = String::from_utf8_lossy(&buffer[..]);
    let mut lines: Vec<&str> = raw_request.split("\r\n").collect();

    let request_line: Vec<&str> = lines[0].split(" ").collect();
    let method = match request_line[0] {
        "GET" => Method::Get,
        "POST" => Method::Post,
        "PUT" => Method::Put,
        "DELETE" => Method::Delete,
        "PATCH" => Method::Patch,
        "HEAD" => Method::Head,
        "CONNECT" => Method::Connect,
        "OPTIONS" => Method::Options,
        "TRACE" => Method::Trace,
        _ => Method::Unknown,
    };

    let route = if request_line.len() == 3 {
        QUERY_PATTERN.replace_all(request_line[1], "").to_string()
    } else {
        "/".to_string()
    };

    // let mut body = String::new();

    lines.remove(0);

    // for line in lines {
    //     if line.contains(": ") {
    //         let split: Vec<&str> = line.split(": ").collect();

    //         headers.insert(
    //             split[0].to_string(),
    //             split[1].to_string()
    //         );
    //     } else {
    //         body += line;
    //     };
    // };
    let headers = utils::parse_key_value_pairs(&raw_request);
    let body = match BODY_PATTERN.captures(&raw_request) {
        None => "".to_string(),
        captures => captures
            .unwrap()
            .get(0)
            .map_or("", |m| m.as_str())
            .to_string(),
    };

    Request {
        headers,
        route,
        method,
        body,
        query: HashMap::new(),
    }
}
