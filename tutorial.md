---
title: Web Calculator with Rust (Yew + Axum + SQLite)
--- 

# Goal

In this tutorial you will learn how to build a **full-stack web calculator using Rust**.

The project contains:

- **Frontend:** Rust + Yew (runs in the browser with WebAssembly)
- **Backend:** Rust + Axum (web server)
- **Database:** SQLite (stores the last calculations)

The calculator can:

- Enter mathematical expressions
- Send them to the backend
- Calculate the result
- Save the last **5 calculations**
- Show the calculation history
- Reset the database

By the end, you will understand how a **Rust frontend communicates with a Rust backend using HTTP APIs**.

---

# Previous Knowledge

This tutorial assumes that you:

- Have **Rust installed**
- Know how to run `cargo run` and `trunk serve`
- Understand **basic Rust syntax**
- Know basic programming concepts (variables, functions)

Helpful but not required:

- Basic idea of **HTTP requests**
- Basic idea of **frontend vs backend**

---

# What you'll learn

In this tutorial you will learn:

- How a **Rust backend with Axum** works
- How to build a **frontend using Yew**
- How to send **HTTP requests with gloo-net**
- How to store data in **SQLite**
- How to create a **simple REST API**
- How frontend and backend **communicate together**

---

# Project Structure

The project has **two Rust programs**:

```

project
│
├─ frontend
│  ├─ src/main.rs
│  └─ Cargo.toml
│
└─ backend
├─ src/main.rs
└─ Cargo.toml

````

- **Frontend:** runs in the browser  
- **Backend:** runs as a local server (`localhost:30000`)

---

# Backend – Axum Server

The backend is responsible for:

- Calculating expressions
- Saving history
- Returning data to the frontend

---

## Step 1 – Import Required Libraries

```rust
use axum::{
    routing::{get, post},
    extract::State,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
````

### Explanation

* **axum** → Web framework
* **serde** → Convert data to/from JSON
* **sqlx** → Database library
* **Arc** → Shared state between requests
* **CorsLayer** → Allows frontend requests

---

## Step 2 – Create the Data Structure

```rust
#[derive(Serialize, Deserialize, Clone)]
struct CalculatorState {
    current_value: f64,
    history: Vec<String>,
}
```

This struct represents the **calculator state**.

It stores:

* The last result
* The last calculations

Example:

```
current_value: 7
history: ["3+4 = 7"]
```

---

## Step 3 – Request Structure

```rust
#[derive(Deserialize)]
struct CalcRequest {
    expression: String,
}
```

The frontend sends something like:

```json
{ "expression": "3+4" }
```

---

## Step 4 – Database Setup

```rust
let db_url = "sqlite://rechner.db?mode=rwc";
let pool = SqlitePool::connect(&db_url).await.expect("DB error");
```

This creates a **SQLite database file**.

Then we create a table:

```rust
sqlx::query(
"CREATE TABLE IF NOT EXISTS CalculatorState
(id INTEGER PRIMARY KEY, current_value REAL, history TEXT)"
)
.execute(&pool)
.await
.unwrap();
```

---

## Step 5 – API Routes

The backend exposes three endpoints:

```
GET  /state
POST /calc
POST /reset
```

Router setup:

```rust
let app = Router::new()
    .route("/calc", post(calculate))
    .route("/state", get(get_state))
    .route("/reset", post(reset_db))
    .layer(CorsLayer::permissive())
    .with_state(shared_state);
```

### Why CORS?

Browsers normally block requests between different origins.

`CorsLayer::permissive()` allows the frontend to talk to the backend.

---

## Step 6 – Start the Server

```rust
let listener = tokio::net::TcpListener::bind("127.0.0.1:30000").await.unwrap();
axum::serve(listener, app).await.unwrap();
```

The backend now runs on:

```
http://127.0.0.1:30000
```

---

# Frontend – Yew

The frontend is a **Rust WebAssembly application**.

It:

* Displays the calculator
* Sends expressions to the backend
* Shows results and history

---

## Step 1 – Import Libraries

```rust
use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
```

Explanation:

* **yew** → frontend framework
* **gloo-net** → HTTP requests
* **serde** → JSON serialization

---

## Step 2 – Shared Data Structure

```rust
#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct FullState {
    current_value: f64,
    history: Vec<String>,
}
```

This must match the **backend structure**.

If they don't match, the program will crash when parsing JSON.

---

## Step 3 – Component State

```rust
let db_state = use_state(|| FullState {
    current_value: 0.0,
    history: vec![]
});

let display = use_state(|| String::new());
```

Two important states exist:

### db_state

Contains:

* last result
* history

### display

Contains what the user typed.

Example:

```
display = "3+4"
```

---

## Step 4 – Load Data on Startup

When the page loads, we request the current state from the backend.

```rust
use_effect_with((), move |_| {
    wasm_bindgen_futures::spawn_local(async move {
        if let Ok(resp) =
            Request::get("http://127.0.0.1:30000/state").send().await
        {
            if let Ok(fetched) = resp.json::<FullState>().await {
                db_state.set(fetched);
            }
        }
    });
    || ()
});
```

This loads the **current database state**.

---

## Step 5 – Add Characters to the Display

```rust
let add_char = move |c: &str| {
    let mut current = (*display).clone();
    current.push_str(c);
    display.set(current);
};
```

Example:

```
Press "3"
display = "3"

Press "+"
display = "3+"
```

---

## Step 6 – Clear the Display

```rust
let clear = move |_| display.set(String::new());
```

Pressing **AC** resets the input.

---

## Step 7 – Send Calculation to Backend

```rust
let calculate = move |_| {
    let expression = (*display).clone();

    wasm_bindgen_futures::spawn_local(async move {
        if let Ok(resp) =
            Request::post("http://127.0.0.1:30000/calc")
                .json(&CalcRequest { expression }).unwrap()
                .send().await
        {
            if let Ok(res) = resp.json::<FullState>().await {
                db_state.set(res);
                display.set(String::new());
            }
        }
    });
};
```

The frontend sends the expression:

```
3+4
```

The backend returns:

```json
{
 "current_value": 7,
 "history": ["3+4 = 7"]
}
```

---

# Result

If everything works correctly:

1. Start backend

```
cargo run
```

2. Start frontend
```
trunk serve
```

4. Open the browser

You should see a **calculator UI**.

Example calculation:

```
3 + 4 = 7
```

History example:

```
2+3 = 5
5*2 = 10
10-4 = 6
```
<img width="782" height="1430" alt="image" src="https://github.com/user-attachments/assets/28797915-e784-4306-a976-d1dee5926bf4" />

The calculator stores the **last 5 operations**.

---

# What Could Go Wrong?

Here are some problems that happened during the project.

---

## SQL Server Error 15405

At first we tried using **Microsoft SQL Server**.

Error:

```
Cannot use the special principal 'sa'
```

This happens because the `sa` user is a **special system account**.

### Solution

We switched to **SQLite**.

SQLite is easier because:

* No server installation
* No users
* No configuration
* Just a file

```
rechner.db
```

---

## Rust Analyzer DATABASE_URL Error

Sometimes VS Code shows errors in SQL queries when using:

```rust
query!
```

Example error:

```
DATABASE_URL not found
```

### Why?

`query!` checks the database **during compile time**.

If the database is not configured yet, it fails.

### Solution

Use:

```rust
sqlx::query()
```

instead of:

```rust
query!
```

It is more flexible during development.

---

## CORS Errors

Sometimes the browser blocks the request.

Example error:

```
CORS policy blocked request
```

### Why?

Frontend and backend run on **different origins**.

### Solution

Enable CORS in Axum:

```rust
.layer(CorsLayer::permissive())
```

This allows the frontend to call the backend.

---

## JSON Type Mismatch

At one point the frontend expected:

```
Vec<f64>
```

But the backend sent:

```
Vec<String>
```

This caused parsing errors.

### Solution

Make sure both sides use the same type:

```rust
history: Vec<String>
```

---

# Conclusion

You successfully built a **full-stack Rust web calculator**.

You learned:

* Rust backend with **Axum**
* Rust frontend with **Yew**
* Sending HTTP requests
* Using **SQLite**
* Creating API endpoints
* Handling common development errors

Possible improvements:

* Add **keyboard input**
* Improve the **UI design**
* Store **more history**
* Add **scientific functions**
* Deploy the backend online



---
