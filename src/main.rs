mod youtube;
mod openai;
use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

use crate::openai::{summarize_recipe, transcribe};

#[derive(Deserialize)]
struct FormData {
    video_id: String,
}

#[post("/summarize")]
async fn summarize(form: web::Form<FormData>) -> impl Responder {
    // Download video
    let download_result = youtube::download(&form.video_id).await;
    if download_result.is_err() {
        return HttpResponse::InternalServerError()
            .content_type("text/plain")
            .body("Failed to download");
    }
    let uri = download_result.unwrap();

    // Transcribe video
    let transcribe_result = transcribe(&uri).await;
    if transcribe_result.is_err() {
        return HttpResponse::InternalServerError()
            .content_type("text/plain")
            .body("Failed to transcribe");
    }
    let transcription = transcribe_result.unwrap();

    // Extract information from transcript
    match summarize_recipe(&transcription).await {
        Ok(summary) => {
            println!("{}", summary);
            HttpResponse::Ok().content_type("application/json").body(summary)
        }
        Err(_) => {
            HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body("Failed to summarize")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Missing .env file");
    HttpServer::new(|| App::new()
        .wrap(
            Cors::default()
                .allowed_origin("http://127.0.0.1:8080")
        )
        .service(summarize))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
