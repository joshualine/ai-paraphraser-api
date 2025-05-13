use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct ParaphraseRequest {
    text: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[post("/paraphrase")]
async fn paraphrase(req: web::Json<ParaphraseRequest>) -> impl Responder {
    let api_key = match env::var("OPENROUTER_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return HttpResponse::InternalServerError().body("OPENROUTER_API_KEY not set");
        }
    };

    let client = Client::new();

    let request_body = ChatRequest {
        model: "mistralai/mistral-7b-instruct".to_string(), // Or another model available on OpenRouter
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful assistant that paraphrases text.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: format!("Paraphrase the following: {}", req.text),
            },
        ],
    };

    let response = match client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "http://localhost:3000") // Set this to your actual domain in production
        .header("X-Title", "AI Paraphraser")
        .json(&request_body)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error sending request: {}", e));
        }
    };

    let json: serde_json::Value = match response.json().await {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error parsing response: {}", e));
        }
    };

    let reply = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Error extracting response");

    HttpResponse::Ok().json(serde_json::json!({ "paraphrased": reply }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(paraphrase)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
