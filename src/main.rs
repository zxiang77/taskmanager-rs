use actix_web::{web, App, Responder, HttpServer, HttpResponse};

fn index(info: web::Path<(String, u32)>) -> impl Responder {
    format!("hello, {}! id: {}", info.0, info.1)
}

fn root() -> impl Responder {
    HttpResponse::Ok().body("bello!")
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(
        web::resource("/{name}/{id}/index.html").to(index)
        )
        .service(web::resource("/").to(root))
    )
        .bind("127.0.0.1:8080")?
        .run()
}
