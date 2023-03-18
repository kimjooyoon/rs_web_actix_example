use actix_web::{
    web,
    body::BoxBody, http::header::ContentType, get, App, HttpResponse, Responder, HttpServer, HttpRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,PartialEq,Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header::ContentType, test, App};

    #[actix_web::test]
    async fn test_hello_ok() {
        let app = test::init_service(
            App::new().service(hello)
        ).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::json())
            .uri("/321")
            .to_request();
        let resp:User = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp, User{ name: "teddy".to_string(), age: 321 })
    }
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