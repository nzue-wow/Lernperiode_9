use axum::{
    routing::{get, post},
    extract::State,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Serialize, Deserialize, Clone)]
struct CalculatorState {
    current_value: f64,
    history: Vec<String>,
}

struct AppState {
    db: SqlitePool,
}

#[derive(Deserialize)]
struct CalcRequest {
    expression: String,
}

#[tokio::main]
async fn main() {
    let db_url = "sqlite://rechner.db?mode=rwc";
    let pool = SqlitePool::connect(&db_url).await.expect("Konnte SQLite nicht laden");

    // Tabelle erstellen
    sqlx::query("CREATE TABLE IF NOT EXISTS CalculatorState (id INTEGER PRIMARY KEY, current_value REAL, history TEXT)")
        .execute(&pool).await.unwrap();
    
    // Startwert
    sqlx::query("INSERT OR IGNORE INTO CalculatorState (id, current_value, history) VALUES (1, 0.0, '[]')")
        .execute(&pool).await.unwrap();

    let shared_state = Arc::new(AppState { db: pool });

    let app = Router::new()
        .route("/calc", post(calculate))
        .route("/state", get(get_state))
        .route("/reset", post(reset_db))
        .layer(CorsLayer::permissive())
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:30000").await.unwrap();
    println!("✅ Backend läuft auf http://127.0.0.1:30000 (SQLite Modus)");
    
    // So startet man den Server in der neuen Axum-Version!
    axum::serve(listener, app).await.unwrap();
}

async fn get_state(State(state): State<Arc<AppState>>) -> Json<CalculatorState> {
    // query statt query! benutzen
    let row = sqlx::query("SELECT current_value, history FROM CalculatorState WHERE id = 1")
        .fetch_one(&state.db)
        .await
        .unwrap();

    // Bei der normalen query-Funktion greifen wir über .get() auf die Spalten zu
    use sqlx::Row; 
    Json(CalculatorState {
        current_value: row.get("current_value"),
        history: serde_json::from_str(&row.get::<String, _>("history")).unwrap_or_default(),
    })
}

async fn calculate(State(state): State<Arc<AppState>>, Json(req): Json<CalcRequest>) -> Json<CalculatorState> {
    let res = meval::eval_str(&req.expression).unwrap_or(0.0);
    
    let current = get_state(State(state.clone())).await;
    let mut new_history = current.history.clone();
    new_history.push(format!("{} = {}", req.expression, res));
    if new_history.len() > 5 { new_history.remove(0); }

    let history_json = serde_json::to_string(&new_history).unwrap();

    // query statt query! und die ? als Platzhalter für SQLite
    sqlx::query("UPDATE CalculatorState SET current_value = ?, history = ? WHERE id = 1")
        .bind(res)
        .bind(history_json)
        .execute(&state.db)
        .await
        .unwrap();

    Json(CalculatorState { current_value: res, history: new_history })
}

async fn reset_db(State(state): State<Arc<AppState>>) -> Json<CalculatorState> {
    sqlx::query("UPDATE CalculatorState SET current_value = 0.0, history = '[]' WHERE id = 1")
        .execute(&state.db)
        .await
        .unwrap();
    Json(CalculatorState { current_value: 0.0, history: vec![] })
}