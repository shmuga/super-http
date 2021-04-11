#[macro_use]
extern crate tantivy;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

use crate::indexer::parser::ParsedFile;
use crate::indexer::HtmlIndexer;
use crate::indexer::HtmlIndexerInternal;
use actix::prelude::*;
use actix::Addr;
use actix_files as afs;
use actix_web::{get, web, Responder};
use actix_web::{App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use std::env;
use std::path::Path;

mod auth;
mod cli;
mod error;
mod indexer;
mod prelude;

#[get("/link/all")]
async fn all_links(
    addr: web::Data<Addr<HtmlIndexer>>,
) -> impl Responder {
    let result = addr.send(indexer::handlers::GetAll()).await;
    let links = result.unwrap().unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(links)
}

#[get("/link/forward/{link}")]
async fn forward_link(
    web::Path(link): web::Path<String>,
    addr: web::Data<Addr<HtmlIndexer>>,
) -> impl Responder {
    let url = urlencoding::decode(&link).unwrap();
    let result = addr.send(indexer::handlers::GetForward(url)).await;
    let links = result.unwrap().unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(links)
}

#[get("/link/backward/{link}")]
async fn backward_link(
    web::Path(link): web::Path<String>,
    addr: web::Data<Addr<HtmlIndexer>>,
) -> impl Responder {
    let url = urlencoding::decode(&link).unwrap();
    let result = addr.send(indexer::handlers::GetBackward(url)).await;
    let links = result.unwrap().unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(links)
}

#[get("/search/{q}")]
async fn search(
    web::Path(q): web::Path<String>,
    addr: web::Data<Addr<HtmlIndexer>>,
) -> impl Responder {
    let result = addr.send(indexer::handlers::SearchAll(q.clone())).await;
    let tags = result.unwrap().unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(tags)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts = crate::cli::get_opts();
    let env_path = Path::new(&opts.input).join("./.env");
    dotenv::from_path(env_path.as_path()).unwrap();

    let port = env::var("PORT").unwrap_or_else(|_|
        portpicker::pick_unused_port()
            .expect("No ports free")
            .to_string(),
    );

    let addr = indexer::HtmlIndexer(HtmlIndexerInternal::new()).start();
    let _ = addr
        .send(indexer::handlers::StartIndexing(opts.clone().input))
        .await;

    println!("Hosted at 0.0.0.0:{}", port);
    HttpServer::new(move || {
        let auth = HttpAuthentication::basic(|r, s| {
            let auth_env = auth::EnvAuth::new();
            auth_env.check_auth(r, s)
        });

        let path = env!("CARGO_MANIFEST_DIR");
        let path = Path::new(path).join("static/search");
        let path = path.to_str().unwrap();

        App::new()
            .service(
                web::scope("/api")
                    .data(addr.clone())
                    .service(search)
                    .service(all_links)
                    .service(forward_link)
                    .service(backward_link)
                    .wrap(auth.clone())
            )
            .service(
                web::scope("")
                    .service(
                        afs::Files::new("/search", path)
                            .show_files_listing()
                            .index_file("search.html")
                            .use_last_modified(true),
                    )
                    .service(
                        afs::Files::new("/", &opts.input)
                            .show_files_listing()
                            .index_file("index.html")
                            .use_last_modified(true),
                    )
                    .wrap(auth),
            )
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
