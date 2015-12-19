
/*

The router's job is to take HTTP requests and parse them into
"ParsedRequest" objects

 */
mod router {
    
    extern crate rustc_serialize;

    use std::fmt;

    use std::error::Error;
    use std::fmt::Display;
    
    use tiny_http::{Server, Request, Response, StatusCode, Method, Header};
    
    use rustc_serialize::json::Json;    
    use rustc_serialize::json::ParserError;

    use std::io::Read;
    use std::io;
    

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
        match get_body_as_json(request) {
            Ok(json) => ParsedRequest::PostJson(json),
            Err(e) => ParsedRequest::BadBody(e),
        }
    }

    fn handle_put(request: &mut Request) -> ParsedRequest {
        match  get_body_as_json(request) {
            Ok(json) => ParsedRequest::PutJson(json),
            Err(e) => ParsedRequest::BadBody(e)
        }
    }

    #[derive(Debug)]
    enum JsonBodyError {
        ReadError(io::Error),
        ParseError(ParserError)
    }
    impl fmt::Display for JsonBodyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                JsonBodyError::ReadError(ref err) => err.fmt(f),
                JsonBodyError::ParseError(ref err) => err.fmt(f)
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
    impl From<ParserError> for JsonBodyError {
        fn from(err: ParserError) -> JsonBodyError {
            JsonBodyError::ParseError(err)
        }
    }

    impl From<io::Error> for JsonBodyError {
        fn from(err: io::Error) -> JsonBodyError {
            JsonBodyError::ReadError(err)
        }
    }


    /// Attempts to construct a Constructs a new `Rc<T>`.
    fn get_body_as_json(request: &mut Request) -> Result<Json, JsonBodyError> {
        let mut content = String::new();
        try!(request.as_reader().read_to_string(&mut content)); //.unwrap();

        let json: Json = try!(Json::from_str(&content));
        //let json: Json = try!(content.parse()); //.unwrap();
        Ok(json)
    }

}
