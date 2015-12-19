
extern crate rustc_serialize;

use rustc_serialize::json::Json;

use tiny_http::{Server, Request, Response, StatusCode, Method, Header};


/*

The router's job is to take HTTP requests and parse them into
"ParsedRequest" objects

*/


enum ParsedRequest {
    GetRequest(String),
    DeleteRequest(String),
    PostJson(Json),
    PutJson(Json),

    UnsupportedMethod,
    BadBody(JsonBodyError),
    BadUrl
}


/// Constructs a new `ParsedRequest` object for the incoming request.
fn parse_request(request: &mut Request) -> ParsedRequest {
    let response = match request.method() {
        &Method::Get => handle_get(request),
        &Method::Post => handle_post(request),
        &Method::Put => handle_put(request), //ParsedRequest::PutJson(get_json(request)),
        &Method::Delete => handle_delete(request), //ParsedRequest::DeleteRequest(request.url().to_string()),
        _ => ParsedRequest::UnsupportedMethod
    };
    response
}


fn handle_get(request: &Request) -> ParsedRequest {
    ParsedRequest::GetRequest(request.url().to_string())
}

fn handle_delete(request: &Request) -> ParsedRequest {
    ParsedRequest::DeleteRequest(request.url().to_string())
}

fn handle_post(request: &mut Request) -> ParsedRequest {
    get_json(request) match {
        Ok(json) => ParsedRequest::PostJson(json),
        Err(e) => ParsedRequest::BadBody(e)
    }
}

fn handle_put(request: &mut Request) -> ParsedRequest {
    get_json(request) match {
        Ok(json) => ParsedRequest::PutJson(json),
        Err(e) => ParsedRequest::BadBody(e)
    }
}


#[derive(Debug)]
enum JsonBodyError {
    ReadError(Read::Error),
    ParseError(Json::Err)
}
impl fmt::Display for JsonBodyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JsonBodyError::ReadError(ref err) => err.fmt(f),
            JsonBodyError::ParseERror(ref err) => err.fmt(f)
        }
    }
}
impl Error for JsonBodyError {
    fn description(&self) -> &str {
        match *self {
            JsonBodyError::ReadError(ref err) => err.description(),
            JsonBodyError::ParseError(ref err) => err.description()
        }
    }
}
impl From<Read::Error> for JsonBodyError {
    fn from(err: Read::Error) -> JsonBodyError {
        JsonBodyError::ReadError(err)
    }
}
impl From<Json::Error> for JsonBodyError {
    fn from(err: Json::Err) -> JsonBodyError {
        JsonBodyError::ParseError(err)
    }
}


/// Attempts to construct a Constructs a new `Rc<T>`.
fn get_body_as_json(request: &mut Request) -> Result<Json, JsonBodyError>
    let mut content = String::new();
    try!(request.as_reader().read_to_string(&mut content)); //.unwrap();
    let json: Json = try!(content.parse::Json()); //.unwrap();
    json
}
