mod config;
mod jobs;

use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/fetch/{config}/{uri}")]
async fn fetch(params: web::Path<(String, String)>) -> impl Responder {
    let job = jobs::fetch::create(&params.0, &params.1);

    let bytes = reqwest::get(&job.uri).await.unwrap().bytes().await.unwrap();

    let process = web::block(move || -> Result<Vec<u8>, ()> {
        let img = image::load_from_memory(&bytes).unwrap();
        let converted = img.resize_to_fill(
            job.width.unwrap() as u32,
            job.height.unwrap() as u32,
            image::imageops::FilterType::Nearest,
        );

        let mut buf: Vec<u8> = vec![];

        converted
            .write_to(&mut buf, image::ImageOutputFormat::Png)
            .unwrap();

        Ok(buf)
    });

    let result = process.await.unwrap();

    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "image/png")
        .body(result)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut args = env::args();
    let path = args
        .nth(1)
        .expect("Please provide a path for the config file");

    let conf = config::parse(&path);

    HttpServer::new(|| App::new().service(fetch))
        .bind(format!("{}:{}", conf.host, conf.port))?
        .run()
        .await
}
