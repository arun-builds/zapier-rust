pub struct Database {
    // conn: Connection
}

impl Default for Database {
    fn default() -> Self {
        Self {  
            // conn
        }
    }
}

impl Database {
    pub fn random_fn(&self) -> String {
        String::from("random_fn_again")
    }
}