use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct FullState {
    current_value: f64,
    history: Vec<String>, // FIX: Muss String sein, da Backend Strings sendet!
}

#[derive(Serialize, Deserialize)]
struct CalcRequest {
    expression: String,
}

#[function_component(App)]
fn app() -> Html {
    let db_state = use_state(|| FullState { current_value: 0.0, history: vec![] });
    let display = use_state(|| String::new());

    // Holt den Stand beim Start
    {
        let db_state = db_state.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // FIX: Route ist /state (nicht /calc)
                if let Ok(resp) = Request::get("http://127.0.0.1:30000/state").send().await {
                    if let Ok(fetched) = resp.json::<FullState>().await {
                        db_state.set(fetched);
                    }
                }
            });
            || ()
        });
    }

    let add_char = {
        let display = display.clone();
        move |c: &str| {
            let mut current = (*display).clone();
            current.push_str(c);
            display.set(current);
        }
    };

    let clear = {
        let display = display.clone();
        move |_| display.set(String::new())
    };

    let calculate = {
        let db_state = db_state.clone();
        let display = display.clone();
        move |_| {
            let expression = (*display).clone();
            let db_state = db_state.clone();
            let display = display.clone();
            
            if !expression.is_empty() {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(resp) = Request::post("http://127.0.0.1:30000/calc")
                        .json(&CalcRequest { expression }).unwrap()
                        .send().await {
                            if let Ok(res) = resp.json::<FullState>().await {
                                db_state.set(res);
                                display.set(String::new());
                            }
                        }
                });
            }
        }
    };

    let reset_db = {
        let db_state = db_state.clone();
        move |_| {
            let db_state = db_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // FIX: Route ist /reset und Methode ist POST
                if let Ok(resp) = Request::post("http://127.0.0.1:30000/reset").send().await {
                    if let Ok(res) = resp.json::<FullState>().await {
                        db_state.set(res);
                    }
                }
            });
        }
    };

    let btn = |label: &'static str, color: &'static str| {
        let add = add_char.clone();
        html! {
            <button onclick={move |_| add(label)} 
                style={format!("padding: 20px; font-size: 1.5rem; border-radius: 50px; border: none; cursor: pointer; background: {}; color: {}; font-weight: bold; transition: 0.2s;", 
                color, if color == "#f1f3f4" { "black" } else { "white" })}>
                { label }
            </button>
        }
    };

    html! {
        <div style="max-width: 350px; margin: 50px auto; font-family: sans-serif; background: #2c3e50; padding: 20px; border-radius: 30px; box-shadow: 0 20px 50px rgba(0,0,0,0.5);">
            
            // NEU: Anzeige der letzten Rechnungen (Historie)
            <div style="text-align: right; color: #bdc3c7; font-size: 0.9rem; min-height: 80px; margin-bottom: 10px; display: flex; flex-direction: column; justify-content: flex-end;">
                { for db_state.history.iter().map(|entry| html! { <div>{ entry }</div> }) }
            </div>

            // Letztes Ergebnis
            <div style="text-align: right; color: #2ecc71; font-size: 1.2rem; min-height: 20px; margin-bottom: 5px; font-weight: bold;">
                { "Letztes Ergebnis: " }{ db_state.current_value }
            </div>

            // Das Eingabe-Display
            <div style="background: #34495e; padding: 20px; border-radius: 15px; text-align: right; font-size: 3rem; margin-bottom: 20px; color: white; overflow: hidden; white-space: nowrap;">
                if (*display).is_empty() { { "0" } } else { { (*display).clone() } }
            </div>

            <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px;">
                <button onclick={clear} style="padding: 20px; font-size: 1.2rem; border-radius: 50px; border: none; cursor: pointer; background: #e74c3c; color: white; font-weight: bold;">{ "AC" }</button>
                { btn("(", "#e0e4e8") }
                { btn(")", "#e0e4e8") }
                { btn("/", "#3498db") }

                { btn("7", "#f1f3f4") }
                { btn("8", "#f1f3f4") }
                { btn("9", "#f1f3f4") }
                { btn("*", "#3498db") }

                { btn("4", "#f1f3f4") }
                { btn("5", "#f1f3f4") }
                { btn("6", "#f1f3f4") }
                { btn("-", "#3498db") }

                { btn("1", "#f1f3f4") }
                { btn("2", "#f1f3f4") }
                { btn("3", "#f1f3f4") }
                { btn("+", "#3498db") }

                { btn("0", "#f1f3f4") }
                { btn(".", "#f1f3f4") }
                <button onclick={calculate} style="padding: 20px; font-size: 1.5rem; border-radius: 50px; border: none; cursor: pointer; background: #2ecc71; color: white; font-weight: bold; grid-column: span 2;">{ "=" }</button>
            </div>

            <button onclick={reset_db} style="width: 100%; margin-top: 20px; padding: 10px; border-radius: 10px; border: none; background: #34495e; color: #e74c3c; cursor: pointer;">
                { "Datenbank Verlauf löschen" }
            </button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}