
use std::collections::HashMap;
use rustc_serialize::json::Json;

#[derive(Debug)]
pub enum TableError {
    TableDoesNotExist,
    TableAlreadyExists,
    KeyAlreadyPresent,
    KeyDoesNotExist,
}


pub struct Tables {
    data: HashMap<String, HashMap<String, Json>>
}

impl Tables {
    pub fn new() -> Tables {
        Tables { data: HashMap::new() }
    }

    pub fn create_table(&mut self, table: &str) -> Result<(), TableError> {
        if (self.data.contains_key(table)) {
            Err(TableError::TableAlreadyExists)
        } else {
            self.data.insert(table.to_string(), HashMap::new());
            Ok(())
        }
    }

    fn get_table(&self, table: &str) -> Result<&HashMap<String, Json>, TableError> {
        match self.data.get(table) {
            Some(table) => Ok(table),
            None => Err(TableError::TableDoesNotExist),
        }
    }

    fn get_mut_table(&mut self, table: &str) -> Result<&mut HashMap<String, Json>, TableError> {
        match self.data.get_mut(table) {
            Some(table) => Ok(table),
            None => Err(TableError::TableDoesNotExist),
        }
    }

    pub fn put(&mut self, table: &str, key: &str, data: Json) -> Result<(), TableError> {

        let table = try!(self.get_mut_table(table));

        if (table.contains_key(key)) {
            Err(TableError::KeyAlreadyPresent)
        } else {
            table.insert(key.to_string(), data);
            Ok(())
        }
    }

    pub fn get(&self, table: &str, key: &str) -> Result<Json, TableError> {

        let table = try!(self.get_table(table));

        match table.get(key) {
            Some(json) => Ok(json.clone()),
            None => Err(TableError::KeyDoesNotExist),
        }
    }

    pub fn delete_key(&mut self, table: &str, key: &str) -> Result<(), TableError> {

        let table = try!(self.get_mut_table(table));

        match table.remove(key) {
            Some(_) => Ok(()),
            None => Err(TableError::KeyDoesNotExist),
        }
    }

    pub fn delete_table(&mut self, table: &str) -> Result<(), TableError> {
        match self.data.remove(table) {
            Some(_) => Ok(()),
            None => Err(TableError::TableDoesNotExist),
        }
    }
}
