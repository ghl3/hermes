

use router::ParsedRequest;

use std::io::Cursor;
use tiny_http::{Server, Request, Response, StatusCode, Method, Header};

use url::Url;

use rustc_serialize::json::Json;

// The handler takes a ParsedRequest and performs a specific
// action on it.
// It returns a response that is sent back to the user

pub fn handle_request(request: ParsedRequest) -> Response<Cursor<Vec<u8>>> {
    match request {
        ParsedRequest::GetRequest(url) => get_handler(url),
        ParsedRequest::DeleteRequest(url) => delete_handler(url),
        ParsedRequest::PostJson(url, json) => post_handler(url, json),
        ParsedRequest::PutJson(url, json) => put_handler(url, json),

        // TODO:  The response MUST include an Allow header containing a list of valid methods for the requested resource.
        ParsedRequest::UnsupportedMethod => http_response(StatusCode(405), "Method not allowed"),
        ParsedRequest::UnknownUrl(url) => http_response(StatusCode(404), "Url does not exist"),

        ParsedRequest::JsonParseError(err) => http_response(StatusCode(400), "Bad Json"),
        ParsedRequest::UrlParseError(err) => http_response(StatusCode(400), "Bad Url"),
    }
}


fn get_handler(url: Url) -> Response<Cursor<Vec<u8>>> {
    let message = format!("GET Url: {}", url);
    println!("{}", message);
    ok(message)
}
fn delete_handler(url: Url) -> Response<Cursor<Vec<u8>>> {
    let message = format!("DELETE Url: {}", url);
    println!("{}", message);
    ok(message)
}
fn post_handler(url: Url, json: Json) -> Response<Cursor<Vec<u8>>> {
    let message = format!("POST Url: {} Json: {}", url, json);
    println!("{}", message);
    ok(message)
}
fn put_handler(url: Url, json: Json) -> Response<Cursor<Vec<u8>>> {
    let message = format!("PUT Url: {} Json: {}", url, json);
    println!("{}", message);
    ok(message)
}

/*
fn json_response(json: Json) -> Response<Cursor<Vec<u8>>> {
    ok(json)
}
*/

fn ok<S>(data: S) -> Response<Cursor<Vec<u8>>> where S: Into<String> {
    http_response(StatusCode(200), data)
}

fn http_response<S>(status: StatusCode, data: S) -> Response<Cursor<Vec<u8>>> where S: Into<String> {

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





