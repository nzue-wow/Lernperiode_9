use axum::{extract::State, routing::{get, put, delete, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;

#[derive(Serialize, Deserialize, Clone)]
struct CalcOperation {
    value: f64,
    operator: Option<String>, // "+", "-", "*", "/"
}

#[derive(Serialize, Deserialize, Clone)]
struct FullState {
    current_value: f64,
    history: Vec<f64>,
}

type SharedState = Arc<RwLock<FullState>>;

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(RwLock::new(FullState {
        current_value: 0.0,
        history: Vec::new(),
    }));

    let app = Router::new()
        .route("/calc", get(get_state))
        .route("/calc", put(calculate)) // Rechnen
        .route("/calc", post(save_current)) // Ergebnis in Historie speichern
        .route("/calc", delete(reset_all))
        .with_state(shared_state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:30000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_state(State(state): State<SharedState>) -> Json<FullState> {
    Json(state.read().unwrap().clone())
}

async fn calculate(State(state): State<SharedState>, Json(payload): Json<CalcOperation>) -> Json<FullState> {
    let mut s = state.write().unwrap();
    if let Some(op) = payload.operator {
        match op.as_str() {
            "+" => s.current_value += payload.value,
            "-" => s.current_value -= payload.value,
            "*" => s.current_value *= payload.value,
            "/" => if payload.value != 0.0 { s.current_value /= payload.value },
            _ => s.current_value = payload.value,
        }
    } else {
        s.current_value = payload.value;
    }
    Json(s.clone())
}

async fn save_current(State(state): State<SharedState>) -> Json<FullState> {
    let mut s = state.write().unwrap();
    let val = s.current_value;
    s.history.push(val);
    Json(s.clone())
}

async fn reset_all(State(state): State<SharedState>) -> Json<FullState> {
    let mut s = state.write().unwrap();
    s.current_value = 0.0;
    s.history.clear();
    Json(s.clone())
}