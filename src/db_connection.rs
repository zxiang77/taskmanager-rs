use postgres::{Client, NoTls};

pub fn get_client() -> Client {
    let conn_str = get_con_str();
    Client::connect(&conn_str, NoTls).unwrap()
}

fn get_con_str() -> String {
//    std::env::var("TASKMNGR_DB_STRING")?
    "postgresql://postgres:mypw@127.0.0.1/".to_owned()
}
