


use std::collections::HashMap;
use rustc_serialize::json::Json;



#[derive(Debug)]
pub enum TableError {
    TableDoesNotExist,
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
}



// static mut _tables:  = HashMap::new();


//fn initialize_tables() -> Tables {
//    HashMap::new()
//}
