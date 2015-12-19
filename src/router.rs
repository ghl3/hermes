
/*
The router's job is to take HTTP requests and parse them into
"ParsedRequest" objects
*/

use std::fmt;

use std::error::Error;
use std::fmt::Display;
use std::io::Read;
use std::io;

use tiny_http::{Server, Request, Response, StatusCode, Method, Header};

use url;
use url::Url;

use rustc_serialize::json;
use rustc_serialize::json::Json;


pub enum ParsedRequest {

    // The Good
    GetRequest(Url),
    DeleteRequest(Url),
    PostJson(Url, Json),
    PutJson(Url, Json),

    // The Bad
    UnsupportedMethod,
    UnknownUrl(Url),

    // The Ugly
    JsonParseError(RequestParseError),
    UrlParseError(RequestParseError),

    //BadBody(RequestParseError),
    //BadUrl(RequestParseError)
}

/// Constructs a new `ParsedRequest` object for the incoming request.
pub fn parse_request(request: &mut Request) -> ParsedRequest {
    let response = match request.method() {
        &Method::Get => handle_get(request),
        &Method::Post => handle_post(request),
        &Method::Put => handle_put(request),
        &Method::Delete => handle_delete(request),
        _ => ParsedRequest::UnsupportedMethod
    };
    response
}

fn handle_get(request: &Request) -> ParsedRequest {
    match get_url(request) {
        Ok(url) => ParsedRequest::GetRequest(url),
        Err(e) => ParsedRequest::UrlParseError(e),
    }
}

fn handle_delete(request: &Request) -> ParsedRequest {
    match get_url(request) {
        Ok(url) => ParsedRequest::DeleteRequest(url),
        Err(e) => ParsedRequest::UrlParseError(e),
    }
}

fn handle_post(request: &mut Request) -> ParsedRequest {
    match get_url(request) {
        Ok(url) => match get_body_as_json(request) {
            Ok(json) => ParsedRequest::PostJson(url, json),
            Err(e) => ParsedRequest::JsonParseError(e),
        },
        Err(e) => ParsedRequest::UrlParseError(e)
    }
}

fn handle_put(request: &mut Request) -> ParsedRequest {
    match get_url(request) {
        Ok(url) => match get_body_as_json(request) {
            Ok(json) => ParsedRequest::PutJson(url, json),
            Err(e) => ParsedRequest::JsonParseError(e),
        },
        Err(e) => ParsedRequest::UrlParseError(e)
    }
}

#[derive(Debug)]
pub enum RequestParseError {
    ReadError(io::Error),
    UrlParseError(url::ParseError),
    JsonParseError(json::ParserError)
}
impl fmt::Display for RequestParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RequestParseError::ReadError(ref err) => err.fmt(f),
            RequestParseError::UrlParseError(ref err) => err.fmt(f),
            RequestParseError::JsonParseError(ref err) => err.fmt(f)
        }
    }
}
impl Error for RequestParseError {
    fn description(&self) -> &str {
        match *self {
            RequestParseError::ReadError(ref err) => err.description(),
            RequestParseError::JsonParseError(ref err) => err.description(),
            RequestParseError::UrlParseError(ref err) => err.description()
        }
    }
}
impl From<io::Error> for RequestParseError {
    fn from(err: io::Error) -> RequestParseError {
        RequestParseError::ReadError(err)
    }
}
impl From<json::ParserError> for RequestParseError {
    fn from(err: json::ParserError) -> RequestParseError {
        RequestParseError::JsonParseError(err)
    }
}
impl From<url::ParseError> for RequestParseError {
    fn from(err: url::ParseError) -> RequestParseError {
        RequestParseError::UrlParseError(err)
    }
}

/// Attempts to construct a Constructs a new `Rc<T>`.
fn get_body_as_json(request: &mut Request) -> Result<Json, RequestParseError> {
    let mut content = String::new();
    try!(request.as_reader().read_to_string(&mut content));
    let json: Json = try!(Json::from_str(&content));
    Ok(json)
}

fn get_url(request: &Request) -> Result<Url, RequestParseError> {
    Ok(try!(Url::parse(request.url())))
}
