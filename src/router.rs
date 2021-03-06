
use std::fmt;

use std::error::Error;
use std::fmt::Display;
use std::io::Read;
use std::io;

use tiny_http::{Request, Method, StatusCode};

use rustc_serialize::json;
use rustc_serialize::json::Json;

use url_parser;
use url_parser::UrlResource;
use std::io::Cursor;
use tiny_http::Response;

use api;
use context::Context;

use http::{ok, http_response};


pub fn handle_request_and_send_response(context: &mut Context, mut request: Request) -> Result<(), io::Error> {
    let response = handle_request(context, &mut request);
    request.respond(response)
}


pub fn handle_request(context: &mut Context, mut request: &mut Request) -> Response<Cursor<Vec<u8>>> {
    match create_response(context, request) {
        Err(RequestError::UnsupportedMethod) => http_response(StatusCode(405), "Method not allowed"),
        Err(RequestError::UnknownResource) => http_response(StatusCode(404), format!("Url does not exist")),
        Err(RequestError::JsonParseError(err)) => http_response(StatusCode(400), format!("Bad Json: {}", err)),
        Err(RequestError::UrlParseError(url)) => http_response(StatusCode(400), format!("Bad Url: {}", url)),
        Err(RequestError::ReadError(err)) => http_response(StatusCode(400), format!("Bad Read: {}", err)),
        Ok(response) => response,
    }
}


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


pub fn create_response(context: &mut Context, mut request: &mut Request) -> Result<Response<Cursor<Vec<u8>>>, RequestError> {

    let url = try!(get_url(request));

    match (request.method().clone(), &url.location[..]) {

        (Method::Post, [ref table]) => Ok(api::post_table(&mut context.tables, table)),

        (Method::Get, [ref table, ref key]) => Ok(api::get_key(&mut context.tables, table, key)),

        (Method::Post, [ref table, ref key]) => {
            let json = try!(get_body_as_json(&mut request));
            Ok(api::post_key_to_table(&mut context.tables, table, key, json))
        },

        (Method::Delete, [ref table]) => Ok(api::delete_table(&mut context.tables, table)),

        (Method::Delete, [ref table, ref key]) => Ok(api::delete_key_from_table(&mut context.tables, table, key)),

        (method, location) => Err(RequestError::UnknownResource),

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
        ("fish", [x, y, z])                => assert!(true),
        _ => assert!(false),
    }
}


#[derive(Debug)]
pub enum RequestError {
    ReadError(io::Error),
    UrlParseError(url_parser::UrlParseError),
    JsonParseError(json::ParserError),
    UnknownResource,
    UnsupportedMethod
}
impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RequestError::ReadError(ref err) => err.fmt(f),
            RequestError::UrlParseError(ref err) => err.fmt(f),
            RequestError::JsonParseError(ref err) => err.fmt(f),
            RequestError::UnknownResource => write!(f, "Unknown Url"), //err.fmt(f),
            RequestError::UnsupportedMethod => write!(f, "UnsupportedMethod"),
        }
    }
}
impl Error for RequestError {
    fn description(&self) -> &str {
        match *self {
            RequestError::ReadError(ref err) => err.description(),
            RequestError::JsonParseError(ref err) => err.description(),
            RequestError::UrlParseError(ref err) => err.description(),
            RequestError::UnknownResource => "Unknown url",
            RequestError::UnsupportedMethod => "Unsupported Method",
        }
    }
}
impl From<io::Error> for RequestError {
    fn from(err: io::Error) -> RequestError {
        RequestError::ReadError(err)
    }
}
impl From<json::ParserError> for RequestError {
    fn from(err: json::ParserError) -> RequestError {
        RequestError::JsonParseError(err)
    }
}
impl From<url_parser::UrlParseError> for RequestError {
    fn from(err: url_parser::UrlParseError) -> RequestError {
        RequestError::UrlParseError(err)
    }
}


/// Attempts to construct a Constructs a new `Rc<T>`.
fn get_body_as_json(request: &mut Request) -> Result<Json, RequestError> {
    let mut content = String::new();
    try!(request.as_reader().read_to_string(&mut content));
    let json: Json = try!(Json::from_str(&content));
    Ok(json)
}


fn get_url(request: &Request) -> Result<UrlResource, RequestError> {
    Ok(try!(url_parser::parse_url_resource(request.url())))
}

