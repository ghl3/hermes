

use tiny_http::{Response, StatusCode, Header};
use std::io::Cursor;

pub fn ok<S>(data: S) -> Response<Cursor<Vec<u8>>> where S: Into<String> {
    http_response(StatusCode(200), data)
}

pub fn http_response<S>(status: StatusCode, data: S) -> Response<Cursor<Vec<u8>>> where S: Into<String> {

    let data = data.into();
    let data_len = data.len();

    Response::new(
        status,
        vec![
            Header::from_bytes(&b"Content-Type"[..], &b"text/plain; charset=UTF-8"[..]).unwrap()
                ],
        Cursor::new(data.into_bytes()),
        Some(data_len),
        None,
        )
}
