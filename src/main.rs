use actix_web::{
    web,
    body::BoxBody, http::header::ContentType, get, App, HttpResponse, Responder, HttpServer, HttpRequest};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    name: String,
    age: u64,
}

impl Responder for User {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok().content_type(ContentType::json())
            .body(body)
    }
}

#[get("/{id}")]
async fn hello(path : web::Path<u64>) -> impl Responder {
    User { name: String::from("teddy"), age: path.into_inner() }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}