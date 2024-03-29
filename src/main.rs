use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::http::header;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

mod ids;
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

#[post("/url")]
async fn shorten_url(repo: Data<UrlRepo>, body: Json<ShortenUrlBody>) -> impl Responder {
    let url = &body.url;
    if Url::parse(url).is_err() {
        return HttpResponse::BadRequest().finish();
    }
    match repo.put(url).await {
        None => HttpResponse::InternalServerError().finish(),
        Some(id) => HttpResponse::Created().json(ShortUrlResponse {
            short_url: ids::encode_id(id),
        }),
    }
}

#[get("/{id}")]
async fn resolve_url(repo: Data<UrlRepo>, path: Path<String>) -> impl Responder {
    match find_url(repo.get_ref(), &path.into_inner()).await {
        None => HttpResponse::NotFound().body("URL not found!"),
        Some(url) => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, url))
            .finish(),
    }
}

#[get("/")]
async fn index() -> impl Responder {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    NamedFile::open(path)
}

async fn find_url(repo: &UrlRepo, path: &str) -> Option<String> {
    let id = ids::decode_id(path)?;
    repo.find(id).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    enable_debug_logs();
    let port: u16 = std::env::var("PORT")
        .map(|p| p.parse().expect("Invalid port!"))
        .unwrap_or(7777);
    let repo: UrlRepo = UrlRepo::new().await.expect("Error creating UrlRepo!");
    println!("Running server on port {port}...");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(Files::new("/static", "./static"))
            .service(resolve_url)
            .service(shorten_url)
            .service(index)
            .app_data(Data::new(repo.clone()))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

fn enable_debug_logs() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
}
