mod openai;
mod youtube;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

use crate::openai::{summarize_recipe, transcribe};

#[derive(Deserialize)]
struct FormData {
    video_id: String,
}

#[post("/summarize")]
async fn summarize(form: web::Form<FormData>) -> impl Responder {
    let download_result = youtube::download(&form.video_id).await;
    if download_result.is_err() {
        return HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("Failed to download");
    }
    let uri = download_result.unwrap();

    let transcribe_result = transcribe(&uri).await;
    if transcribe_result.is_err() {
        transcribe_result.unwrap();
        return HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("Failed to transcribe");
    }
    let transcription = transcribe_result.unwrap();

    match summarize_recipe(&transcription).await {
        Ok(summary) => {
            println!("{}", summary);
            HttpResponse::Ok().content_type("text/plain").body(summary)
        }
        Err(err) => {
            println!("{}", err);
            HttpResponse::BadRequest()
                .content_type("text/plain")
                .body("Failed to summarize")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Missing .env file");
    HttpServer::new(|| App::new().service(summarize))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
