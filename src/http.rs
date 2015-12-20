
use std::fmt;
use std::io::Cursor;


pub struct ParsedResponse {
    pub status: u32,
    pub data: Cursor<Vec<u8>>
}

impl ParsedResponse {
    pub fn new<D>(status: u32, data: D) -> ParsedResponse where D: Into<Vec<u8>> {
        let data = data.into();
        ParsedResponse { status: status, data: Cursor::new(data) }
    }
}

impl fmt::Display for ParsedResponse {
    pub fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParsedResponse({:?} {:?})", self.status, self.data)
    }
}
