
/*
The router's job is to take HTTP requests and parse them into
"ParsedRequest" objects
*/

use std::fmt;

use std::error::Error;
use std::fmt::Display;
use std::io::Read;
use std::io;

use tiny_http::{Request, Method};

//use url;
//use url::Url;

use rustc_serialize::json;
use rustc_serialize::json::Json;

use url_parser;
use url_parser::UrlResource;

use handler::{ok, http_response};

use std::io::Cursor;
use tiny_http::Response;

//use http::ParsedResponse;

use http;

pub enum ParsedRequest {

    // The Good
    GetRequest(UrlResource),
    DeleteRequest(UrlResource),
    PostJson(UrlResource, Json),
    PutJson(UrlResource, Json),

    // The Bad
    UnsupportedMethod,
    UnknownUrl(UrlResource),

    // The Ugly
    JsonParseError(RequestParseError),
    UrlParseError(RequestParseError),
}


/*
pub fn request_router(mut request: &mut Request) -> ParsedResponse {

    let url = try!(get_url(request));

    match (request.method, url.location) {
    }
}
 */


/*

Hermes API:

Post /table
{name: name, ...}
- Create a table with name <name>

Post /table/<name>/key
{data}
- Create a key in table with name <name> with value <data>
- 400 if table doesn't exist
- 400 if key already exists

PUt /table/<name>/key
{data}
- Update a key in table with name <name> with value <data>
- 400 if table doesn't exist
- 400 if key doesn't exist

Get /table/<name>/<key>
- Return the data for a given key
- 400 if table doesn't exist
- 400 if key doesn't exist

*/


pub fn request_router(method: &Method, url: UrlResource) -> Response<Cursor<Vec<u8>>> {
    match (method, &url.location[..]) {
        (&Method::Post, [ref table]) => ok("foobar"),
        _ => ok("baz"),
    }
}

#[test]
fn test_routing() {
    let response = request_router(&Method::Post, UrlResource::from_resource("/foo").unwrap());
    assert!(http::response_string(response) == http::response_string(ok("foobar")));
}

#[test]
fn test_vector_match() {

    let v = vec!("foo", "bar", "baz");

    match ("fish", &v[..]) {
        (_, [])                       => assert!(false),
        (_, [elem])                   => assert!(false),
//        [first, second, ..rest]  => println!("{:?}", rest),  // => &[3, 4, 5]
        ("fish", [x, y, z])                => assert!(true),
        _ => assert!(false),
    }
}




/// Constructs a new `ParsedRequest` object for the incoming request.
pub fn parse_request(mut request: &mut Request) -> ParsedRequest {
    match request.method() {
        &Method::Get => handle_get(&request),
        &Method::Post => handle_post(&mut request),
        &Method::Put => handle_put(&mut request),
        &Method::Delete => handle_delete(&request),
        _ => ParsedRequest::UnsupportedMethod,
    }
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
    UrlParseError(url_parser::UrlParseError),
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
impl From<url_parser::UrlParseError> for RequestParseError {
    fn from(err: url_parser::UrlParseError) -> RequestParseError {
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


fn get_url(request: &Request) -> Result<UrlResource, RequestParseError> {
    Ok(try!(url_parser::parse_url_resource(request.url())))
    //let url_string = format!("http://X.com/{}", request.url());
    //println!("Found url: {}", url_string);
    //Ok(try!(Url::parse(&url_string[..])))
}
