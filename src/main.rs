use actix_web::{web, App, Responder, HttpServer, HttpResponse};
use std::cell::Cell;

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

fn main() -> std::io::Result<()> {
    // could use .configure(Fn) as well to set testing routes
    HttpServer::new(|| App::new()
        .data(AppState::new())
        .service(
        web::resource("/{name}/{id}/index.html").to(index)
        )
        .service(web::resource("/").to(root))
    )
        .bind("127.0.0.1:8080")?
        .run()
}
