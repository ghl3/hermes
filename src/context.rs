

use table::Tables;


pub struct Context {
    pub tables: Tables
}
impl Context {
    pub fn new(tables: Tables) -> Context {
        Context { tables: tables }
    }
}
