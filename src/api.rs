

use std::io::Cursor;
use rustc_serialize::json::Json;
use tiny_http::{Response};

use http::{ok, http_response};

pub fn post_table(table: &str, data: Json) -> Response<Cursor<Vec<u8>>> {
    ok(format!("Creating table {}", table))
}


pub fn get_key(table: &str, key: &str) -> Response<Cursor<Vec<u8>>> {
    ok(format!("Geting key {} from table {}", key, table))
}

