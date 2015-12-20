
use std::collections::HashMap;
use std::vec::Vec;


pub struct UrlResource<'a> {
    location: Vec<&'a str>,
    params: HashMap<String, String>
}


pub enum ParseError {
    InvalidUrl,
    InvalidQuery
}


impl<'a> UrlResource<'a> {
    fn new(location: Vec<&'a str>, params: HashMap<String, String>) -> UrlResource<'a> {
        UrlResource { location: location, params: params }
    }

    fn parse_location(location: &'a str) -> Result<Vec<&'a str>, ParseError> {
        if (location.len() == 0) {
            Err(ParseError::InvalidUrl)
            //        } else if (location.char_at(0) != '/') {
        } else if (location.chars().nth(0).unwrap() != '/') {
            Err(ParseError::InvalidUrl)
        } else {
            Ok(location[1..].split("/").collect())
        }
    }

    fn parse_query(query: &str) -> Result<HashMap<String, String>, ParseError> {
        Ok(HashMap::new())
    }

    fn from_resource(location: &'a str) -> Result<UrlResource<'a>, ParseError> {
        match UrlResource::parse_location(location) {
            Ok(loc_vec) => Ok(UrlResource::new(loc_vec, HashMap::new())),
            Err(e) => Err(e)
        }
    }

    fn from_resource_and_query(location: &'a str, query: &str) -> Result<UrlResource<'a>, ParseError> {
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


pub fn parse_url_resource(url: &str) -> Result<UrlResource, ParseError> {

    let url_split = url.split("?").collect::<Vec<&str>>();

    match url_split.len() {
        0 => Err(ParseError::InvalidUrl),
        1 => UrlResource::from_resource(url_split[0]),
        2 => UrlResource::from_resource_and_query(url_split[0], url_split[1]),
        _ => Err(ParseError::InvalidUrl),
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
