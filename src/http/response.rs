use crate::http::status::{get_status, StatusCode};
use std::collections::HashMap;

pub struct Response {
    pub headers: HashMap<String, String>,
    pub body: String,
    pub status_code: StatusCode,
}

impl Default for Response {
    fn default() -> Response {
        Response {
            headers: HashMap::new(),
            body: String::new(),
            status_code: StatusCode::OK,
        }
    }
}

pub fn create(mut response: Response) -> Vec<u8> {
    response.headers.insert(
        "Content-Length".to_string(),
        (response.body.len() + 2).to_string(),
    );
    let mut headers = String::new();
    for (name, value) in response.headers.iter() {
        headers += &format!("{}: {}\r\n", name, value);
    }

    format!(
        "HTTP/1.1 {}\r\n{}\r\n\r\n{}",
        get_status(response.status_code),
        headers,
        response.body
    )
    .as_bytes()
    .to_owned()
}
