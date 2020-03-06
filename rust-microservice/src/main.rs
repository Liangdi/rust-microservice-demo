use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Message {
    msg: String,
    code: i32,
}


async fn index() -> impl Responder {
    HttpResponse::Ok().body("this is a rust microservice demo")
}

async fn plain_text_resp() -> impl Responder {
    HttpResponse::Ok().body("from /foo")
}

async fn json_resp() -> impl Responder {
    let p = Message {
        msg: "from /bar".to_string(),
        code: 0,
    };

    HttpResponse::Ok().json(p)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("listening at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/foo", web::get().to(plain_text_resp))
            .route("/bar",web::get().to(json_resp))
    })
        .bind("localhost:8080")?
        .run()
        .await
}