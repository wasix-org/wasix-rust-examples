use axum::{
    body::Body, extract::Request, http::StatusCode, response::Response, routing::any, Router,
};

async fn handle(req: Request) -> Response {
    let url = format!(
        "https://www.rust-lang.org{}",
        req.uri()
            .path_and_query()
            .map(|p| p.as_str())
            .unwrap_or("/")
    );

    let (status, body) = match fetch(&url).await {
        Ok(body) => (StatusCode::OK, body),
        Err(err) => (
            err.status().unwrap_or(StatusCode::BAD_GATEWAY),
            err.to_string(),
        ),
    };

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
