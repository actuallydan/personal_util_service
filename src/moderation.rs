use dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use tide::prelude::json;

use surf;
use tide::{Request, Response};

#[derive(Debug, Deserialize, Serialize)]
pub struct ModerationResult {
    id: String,
    model: String,
    results: Vec<Moderation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Moderation {
    flagged: bool,
    categories: Categories,
    category_scores: CategoryScores,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Categories {
    sexual: bool,
    hate: bool,
    violence: bool,
    #[serde(rename = "self-harm")]
    self_harm: bool,
    #[serde(rename = "sexual/minors")]
    sexual_minors: bool,
    #[serde(rename = "hate/threatening")]
    hate_threatening: bool,
    #[serde(rename = "violence/graphic")]
    violence_graphic: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryScores {
    sexual: f64,
    hate: f64,
    violence: f64,
    #[serde(rename = "self-harm")]
    self_harm: f64,
    #[serde(rename = "sexual/minors")]
    sexual_minors: f64,
    #[serde(rename = "hate/threatening")]
    hate_threatening: f64,
    #[serde(rename = "violence/graphic")]
    violence_graphic: f64,
}
#[derive(Debug, Deserialize, Serialize)]
struct ModerationBodyInput {
    input: String,
}

pub async fn get_recommendation(mut req: Request<()>) -> tide::Result {
    dotenv::dotenv().ok();

    let input: ModerationBodyInput = req.body_json().await?;

    let data: ModerationResult = surf::post("https://api.openai.com/v1/moderations")
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                env::var("OPEN_AI_KEY").unwrap_or(String::from("lmao"))
            ),
        )
        .body_json(&input)?
        .recv_json()
        .await?;

    Ok(Response::from(json!({ "data": data })))
}
