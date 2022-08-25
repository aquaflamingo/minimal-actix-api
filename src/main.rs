use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
struct Track {
    name: String,
    artist: String,
}


#[get("/")]
async fn index() -> impl Responder {
    let track = Track{name: String::from("Expansions"), artist: String::from("Lonnie Liston Smith")};

    HttpResponse::Ok().json(track)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
