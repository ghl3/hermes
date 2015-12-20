
use std::io::Cursor;

use rustc_serialize::json;
use rustc_serialize::json::Json;
use tiny_http::{Response, StatusCode};

use http::{ok, okJson, http_response};

use table::Tables;
use table::TableError;


pub fn post_table(tables: &mut Tables, table: &str) -> Response<Cursor<Vec<u8>>> {
    println!("Creating Table: {}", table);
    match tables.create_table(table) {
        Err(e) => handle_table_error(e),
        Ok(_) => ok(format!("Successfully Created Table: {}", table))
    }
}

pub fn post_key_to_table(tables: &mut Tables, table: &str, key: &str, data: Json) -> Response<Cursor<Vec<u8>>> {
    match tables.put(table, key, data) {
        Err(e) => handle_table_error(e),
        Ok(_) => ok(format!("Successfully added key: {} to table {}", key, table))
    }
}

pub fn get_key(tables: &mut Tables, table: &str, key: &str) -> Response<Cursor<Vec<u8>>> {
    match tables.get(table, key) {
        Err(e) => handle_table_error(e),
        Ok(json) => okJson(json),
    }
}

pub fn delete_key_from_table(tables: &mut Tables, table: &str, key: &str) -> Response<Cursor<Vec<u8>>> {
    match tables.delete_key(table, key) {
        Err(e) => handle_table_error(e),
        Ok(_) => ok(format!("Successfully deleted key: {} from table: {}", key, table)),
    }
}

pub fn delete_table(tables: &mut Tables, table: &str) -> Response<Cursor<Vec<u8>>> {
    match tables.delete_table(table) {
        Err(e) => handle_table_error(e),
        Ok(_) => ok(format!("Successfully deleted table: {}", table)),
    }
}


fn handle_table_error(error: TableError) -> Response<Cursor<Vec<u8>>> {
    match error {
        TableError::TableDoesNotExist => http_response(StatusCode(400), "Table does not exist"),
        TableError::TableAlreadyExists => http_response(StatusCode(400), "Table already exists"),
        TableError::KeyAlreadyPresent => http_response(StatusCode(400), "Key already present"),
        TableError::KeyDoesNotExist => http_response(StatusCode(400), "Key does not exist"),
    }
}
