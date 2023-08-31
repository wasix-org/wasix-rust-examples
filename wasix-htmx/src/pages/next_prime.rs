use crate::pages::HtmlTemplate;
use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use std::sync::{atomic::AtomicU64, Arc};

pub struct PrimeState(AtomicU64);

impl PrimeState {
    pub fn new() -> Self {
        PrimeState(AtomicU64::new(0))
    }
}

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {
    pub next_prime_number: u64,
}

pub async fn view(State(state): State<Arc<PrimeState>>) -> impl IntoResponse {
    let template = HomeTemplate {
        next_prime_number: state.0.load(std::sync::atomic::Ordering::Relaxed),
    };
    HtmlTemplate::new(template)
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn next_prime_number(n: u64) -> u64 {
    let mut n = n + 1;
    loop {
        if is_prime(n) {
            return n;
        }
        n += 1;
    }
}

pub async fn calculate_next_prime(State(state): State<Arc<PrimeState>>) -> impl IntoResponse {
    let previous = state.0.load(std::sync::atomic::Ordering::Relaxed);
    let next_prime = next_prime_number(previous);
    state
        .0
        .store(next_prime, std::sync::atomic::Ordering::Relaxed);

    Html(format!("{}", next_prime))
}
