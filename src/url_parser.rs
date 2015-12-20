
use std::fmt;

use std::collections::HashMap;
use std::vec::Vec;
use std::borrow::ToOwned;
use std::fmt::Display;
use std::error::Error;


//#[derive(Display)]
pub struct UrlResource {
    location: Vec<String>,
    params: HashMap<String, String>
}
impl fmt::Display for UrlResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.location, self.params)
    }
}


#[derive(Debug)]
pub enum UrlParseError {
    InvalidUrl,
    InvalidQuery
}
impl fmt::Display for UrlParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UrlParseError::InvalidUrl => write!(f, "InvalidUrl"),
            UrlParseError::InvalidQuery => write!(f, "InvalidQuery"),
        }
    }
}
impl Error for UrlParseError {
    fn description(&self) -> &str {
        match *self {
            UrlParseError::InvalidUrl => "InvalidUrl",
            UrlParseError::InvalidQuery => "InvalidQuery",
        }
    }
}


impl UrlResource {
    fn new(location: Vec<String>, params: HashMap<String, String>) -> UrlResource {
        UrlResource { location: location, params: params }
    }

    fn parse_location(location: &str) -> Result<Vec<String>, UrlParseError> {
        if (location.len() == 0) {
            Err(UrlParseError::InvalidUrl)
        } else if (location.chars().nth(0).unwrap() != '/') {
            Err(UrlParseError::InvalidUrl)
        } else {
            Ok(location[1..].split("/").map(ToOwned::to_owned).collect::<Vec<String>>())
        }
    }

    fn parse_query(query: &str) -> Result<HashMap<String, String>, UrlParseError> {
        if (query == "") {
            Ok(HashMap::new())
        } else {

            let mut query_map = HashMap::new();

            for param_pair in query.split("&").collect::<Vec<&str>>() {

                let param_pair_split = param_pair.split("=").collect::<Vec<&str>>();

                if (param_pair_split.len() != 2) {
                    return Err(UrlParseError::InvalidUrl);
                } else {
                    query_map.insert(ToOwned::to_owned(param_pair_split[0]),
                                     ToOwned::to_owned(param_pair_split[1]));
                }
            }
            Ok(query_map)
        }
    }

    fn from_resource(location: &str) -> Result<UrlResource, UrlParseError> {
        match UrlResource::parse_location(location) {
            Ok(loc_vec) => Ok(UrlResource::new(loc_vec, HashMap::new())),
            Err(e) => Err(e)
        }
    }

    fn from_resource_and_query(location: &str, query: &str) -> Result<UrlResource, UrlParseError> {
        match UrlResource::parse_location(location) {
            Ok(loc_vec) => {
                match UrlResource::parse_query(query) {
                    Ok(query_map) => Ok(UrlResource::new(loc_vec, query_map)),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }
}


pub fn parse_url_resource(url: &str) -> Result<UrlResource, UrlParseError> {

    let url_split = url.split("?").collect::<Vec<&str>>();

    match url_split.len() {
        0 => Err(UrlParseError::InvalidUrl),
        1 => UrlResource::from_resource(url_split[0]),
        2 => UrlResource::from_resource_and_query(url_split[0], url_split[1]),
        _ => Err(UrlParseError::InvalidUrl),
    }
}


#[test]
fn it_works() {
    match parse_url_resource("/foo/bar") {
        Ok(rs) => {
            println!("location: {:?} params: {:?}", rs.location, rs.params);
            assert!(rs.location == vec!("foo", "bar"));
        },
        Err(_) => assert!(false),
    }
}


#[test]
fn it_works_2() {
    match parse_url_resource("/foo/bar?a=A&b=B") {
        Ok(rs) => {
            println!("location: {:?} params: {:?}", rs.location, rs.params);
            assert!(rs.location == vec!("foo", "bar"));
            assert!(rs.params == HashMap::new());
        },
        Err(_) => assert!(false),
    }
}
