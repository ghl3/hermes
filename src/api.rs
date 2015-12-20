
use std::io::Cursor;

use rustc_serialize::json;
use rustc_serialize::json::Json;
use tiny_http::{Response};

use http::{ok, okJson, http_response};

use table::Tables;


pub fn post_table(tables: &mut Tables, table: &str) -> Response<Cursor<Vec<u8>>> {
    ok(format!("Creating table {}", table))
}

pub fn post_key_to_table(tables: &mut Tables, table: &str, key: &str, data: Json) -> Response<Cursor<Vec<u8>>> {
    ok(format!("Creating key {} on table: {} with data: {:?}", key, table, data))
}

//pub fn post_table(table: &str) -> Response<Cursor<Vec<u8>>> {
//    ok(format!("Creating table {}", table))
//}

pub fn get_key(tables: &mut Tables, table: &str, key: &str) -> Response<Cursor<Vec<u8>>> {
    okJson(Json::from_str("{\"foo\":\"bar\"}").unwrap())
}



