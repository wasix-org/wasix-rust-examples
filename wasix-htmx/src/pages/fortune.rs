use axum::response::{Html, IntoResponse};

#[derive(serde::Deserialize)]
struct FortuneResponse {
    fortune: String,
}

async fn fetch_fortune() -> anyhow::Result<String> {
    let fortune = reqwest::get("http://yerkee.com/api/fortune/wisdom")
        .await?
        .json::<FortuneResponse>()
        .await?;

    Ok(fortune.fortune)
}

pub async fn fortune() -> impl IntoResponse {
    // fetch fortune
    let fortune = match fetch_fortune().await {
        Ok(fortune) => fortune,
        Err(err) => format!("Failed to fetch fortune. Error: {}", err),
    };
    Html(format!("\"{}\"", fortune))
}
