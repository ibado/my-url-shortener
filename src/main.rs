use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

mod urls;
use crate::urls::UrlRepo;

#[derive(Deserialize)]
struct ShortenUrlBody {
    url: String,
}

#[derive(Serialize)]
struct ShortUrlResponse {
    short_url: String,
}

#[post("/")]
async fn shorten_url(db: Data<UrlRepo>, body: Json<ShortenUrlBody>) -> impl Responder {
    let id = create_id();
    db.put(&id, &body.url);
    HttpResponse::Created().json(ShortUrlResponse { short_url: id })
}

#[get("/{id}")]
async fn resolve_url(db: Data<UrlRepo>, path: Path<String>) -> impl Responder {
    let id = path.into_inner();
    match db.find(&id) {
        None => HttpResponse::NotFound().body("URL not found!"),
        Some(url) => HttpResponse::PermanentRedirect()
            .insert_header(("Location", url))
            .finish(),
    }
}

fn create_id() -> String {
    "asd124".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    enable_debug_logs();
    let port = 7777;
    let repo: UrlRepo = UrlRepo::new();
    println!("Running server on port {port}...");
    HttpServer::new(move || {
        App::new()
            .service(resolve_url)
            .service(shorten_url)
            .app_data(Data::new(repo.clone()))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

fn enable_debug_logs() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
}
