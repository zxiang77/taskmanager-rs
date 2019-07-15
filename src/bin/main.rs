use actix_web::{web, App, Responder, HttpServer, HttpResponse};
use std::cell::Cell;
use actix_web::middleware::Logger;
use taskmanager::task::Task;
use taskmanager::db_connection;
use taskmanager::db_connection::get_client;
//#[macro_use] extern crate log;
//extern crate env_logger;

#[macro_use]
pub extern crate num_derive;

fn index(info: web::Path<(String, u32)>) -> impl Responder {
    format!("hello, {}! id: {}", info.0, info.1)
}

fn root() -> impl Responder {
    HttpResponse::Ok().body("bello!")
}

struct AppState {
    visitor_count: Cell<i32>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            visitor_count: Cell::new(0)
        }
    }
}

fn main() -> std::io::Result<()> { // Use
    // could use .configure(Fn) as well to set testing routes
    std::env::set_var("RUST_LOG", "actix_web=info");
//    env_logger::init();

    let c =  get_client();

    HttpServer::new(|| App::new()
        .data(AppState::new())
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .service(
        web::resource("/{name}/{id}/index.html").to(index)
        )
        .service(web::resource("/").to(root))
    )
        .bind("127.0.0.1:8080")?
        .run()
}
// https://stackoverflow.com/questions/750604/freeing-up-a-tcp-ip-port free up port