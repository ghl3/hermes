

use tiny_http::{Response, StatusCode, Header};
use std::io::Cursor;


pub fn ok<S>(data: S) -> Response<Cursor<Vec<u8>>> where S: Into<String> {
    http_response(StatusCode(200), data)
}

/*
pub fn http_response<S>(status: StatusCode, data: S) -> http::ParsedResponse where S: Into<String> {
    let data = data.into();
    http::ParsedResponse::new(status, data)
}
*/

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




/*
use std::io::Read;
use std::fmt;
use std::io::Cursor;

use tiny_http;
use tiny_http::{StatusCode, Response, HTTPVersion};

pub struct ParsedResponse {
    pub status: StatusCode,
    pub data: Cursor<Vec<u8>>,
}

impl ParsedResponse {
    pub fn new<D>(status: StatusCode, data: D) -> ParsedResponse where D: Into<Vec<u8>> {
        let data = data.into();
        ParsedResponse { status: status, data: Cursor::new(data) }
    }
}

impl fmt::Display for ParsedResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParsedResponse({:?} {:?})", self.status, self.data)
    }
}




pub fn response_string<R>(response: Response<R>) -> String where R: Read  {
    / *
fn raw_print<W: Write>(self, writer: W, http_version: HTTPVersion, request_headers: &[Header], do_not_send_body: bool, upgrade: Option<&str>) -> IoResult<()>
* /
    //let mut s = String::new();
    let mut buff = Cursor::new(vec![0; 15]);
    response.raw_print(&mut buff, HTTPVersion::from((1, 0)), &[], false, None);
    String::from_utf8(buff.into_inner()).unwrap()
}


*/
