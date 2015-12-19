

use std::collections::HashMap;
use std::vec::Vec;
//use  std::error::Error;

pub struct UrlResource<'a> {
    location: Vec<&'a str>,
    params: HashMap<String, String>
}

//#[deriving(Debug)]
pub enum ParseError {
    InvalidUrl,
    InvalidQuery
}
/*
impl fmt::Display for ParseError {
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
*/

impl<'a> UrlResource<'a> {
    fn new(location: Vec<&'a str>, params: HashMap<String, String>) -> UrlResource<'a> {
        UrlResource { location: location, params: params }
    }
    fn from_resource(location: &'a str) -> UrlResource<'a> {
        UrlResource::new(location.split("/").collect::<Vec<&str>>(),  HashMap::new())
    }
    fn from_resource_and_query(location: &'a str, query: &str) -> UrlResource<'a> {
        UrlResource::new(location.split("/").collect::<Vec<&str>>(), parse_query(query))
    }
}

pub fn parse_url_resource(url: &str) -> Result<UrlResource, ParseError> {

    let url_split = url.split("?").collect::<Vec<&str>>();

    match url_split.len() {
        0 => Err(ParseError::InvalidUrl),
        1 => Ok(UrlResource::from_resource(url_split[0])), //(resource.split("/"), HashMap.new())),
        2 => Ok(UrlResource::from_resource_and_query(url_split[0], url_split[1])),
        _ => Err(ParseError::InvalidUrl),
    }
    /*
    match resource.split("?").collect().as_slice() {
        [resource] => Ok(UrlResource::new(resource.split("/"), HashMap.new())),
        [resource, params] => Ok(UrlResource::new(resource.split("/"), parse_params(params))),
        _ => Err("Foobar")

    }
*/
}


pub fn parse_query(query: &str) -> HashMap<String, String> {
    HashMap::new()
}
