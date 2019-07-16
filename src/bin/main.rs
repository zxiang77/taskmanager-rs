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

use std::fs;
fn dev() -> impl Responder {
    println!("reading from index.html");

    // todo find why the info: web::Form<DevReq> args doesn't work
//    println!("reading from the request {}", info.sql);

    let contents = fs::read_to_string("./static/index.html").expect("Can't read file.");
    HttpResponse::Ok().body(&contents)
}

use actix_web::{Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct DevReq {
    sql: String,
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

fn main() { // Use
    // could use .configure(Fn) as well to set testing routes
//    std::env::set_var("RUST_LOG", "actix_web=info");
//    env_logger::init();

    let mut c =  get_client();
//    let res = c.execute("CREATE DATABASE test;", &[]);
//    "CREATE DATABASE test (
//        ID serial PRIMARY KEY,
//        NAME VARCHAR(50) UNIQUE NOT NULL,
//        DESC VARCHAR(150) UNIQUE NOT NULL,
//        STATE INTEGER NOT NULL,
//        DUE_DATE TIMESTAMP WITHOUT TIME ZONE
//    )"
//
//    match res {
//        Ok(v) => println!("{}", format!("the result is {}", v)),
//        Err(e) => println!("{}", format!("the error is {}", e))
//    }
    start(8080);
}

fn start(port: i32) {
    let server = HttpServer::new(|| App::new()
        .data(AppState::new())
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
//        .service(
//        web::resource("/{name}/{id}/index.html").to(index)
//        )
        .service(web::resource("/").to(root))
        .service(web::resource("/dev").route(web::post()).to(dev))
    )
        .bind(format!("127.0.0.1:{}", port));
//    let h = server.unwrap();
    match server {
        Ok(svr) => {
            println!("Server started with port {}", port);
            svr.run();
            ()
        },
        Err(_) => {
            // kill -9 $(lsof -n -i :'8080' | grep LISTEN | awk '{print $2}')
            use std::process::Command;
            Command::new("sh")
                .arg("-c")
                .arg("kill -9 $(lsof -n -i :'8080' | grep LISTEN | awk '{print $2}')")
                .output()
                .expect("Unable to run the command.");
            start(port);
            ()

        }
    };
}

// https://stackoverflow.com/questions/750604/freeing-up-a-tcp-ip-port free up port
// lsof -n -i :'8080' | grep LISTEN