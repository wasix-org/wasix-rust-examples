use axum::{body::Body, http::StatusCode, response::Response, routing::any, Router};

mod news_scraper;

async fn handle() -> Response {
    let url = "https://news.ycombinator.com/";

    let (status, page) = match fetch(url).await {
        Ok(page) => (StatusCode::OK, page),
        Err(err) => {
            let status = err.status().unwrap_or(StatusCode::BAD_GATEWAY);
            return Response::builder()
                .status(status)
                .body(Body::from(err.to_string()))
                .unwrap();
        }
    };

    let mut news = news_scraper::NewsScraper::new();
    news.scrape(page);
    let body = news.get_news();

    Response::builder()
        .status(status)
        .body(Body::from(body))
        .unwrap()
}

async fn fetch(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url).await?.text().await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // check if there's an environment variable for the port
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port.parse::<u16>()?;

    let app = Router::new().fallback(any(handle));
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await?;
    println!("Listening on http://127.0.0.1:{port}");
    axum::serve(listener, app).await?;
    Ok(())
}
