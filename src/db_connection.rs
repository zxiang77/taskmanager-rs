use postgres::{Client};

impl Client {
    pub fn new() -> Self {
        let conn_str = std::env::var("TASKMNGR_DB_STRING")?;
        Client::connect(&conn_str, &[])?
    }
}
