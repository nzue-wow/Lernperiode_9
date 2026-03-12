use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct FullState {
    current_value: f64,
    history: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
struct CalcOperation {
    value: f64,
    operator: Option<String>,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| FullState { current_value: 0.0, history: vec![] });
    let input_ref = use_node_ref();

    // Zentrale Funktion für API-Aufrufe
    let send_op = {
        let state = state.clone();
        let input_ref = input_ref.clone();
        move |op: Option<&str>| {
            let state = state.clone();
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            let val = input.value().parse::<f64>().unwrap_or(0.0);
            let op_string = op.map(|s| s.to_string());

            wasm_bindgen_futures::spawn_local(async move {
                let res: FullState = Request::put("http://127.0.0.1:30000/calc")
                    .json(&CalcOperation { value: val, operator: op_string }).unwrap()
                    .send().await.unwrap().json().await.unwrap();
                state.set(res);
                input.set_value(""); // Input leeren nach Eingabe
            });
        }
    };

    let save_result = {
        let state = state.clone();
        move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res: FullState = Request::post("http://127.0.0.1:30000/calc")
                    .send().await.unwrap().json().await.unwrap();
                state.set(res);
            });
        }
    };

    let reset = {
        let state = state.clone();
        move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res: FullState = Request::delete("http://127.0.0.1:30000/calc")
                    .send().await.unwrap().json().await.unwrap();
                state.set(res);
            });
        }
    };

    html! {
        <div class="calculator-container" style="max-width: 400px; margin: 50px auto; font-family: 'Segoe UI', sans-serif; background: #2c3e50; padding: 25px; border-radius: 20px; box-shadow: 0 10px 30px rgba(0,0,0,0.3); color: white;">
            <h1 style="text-align: center; margin-top: 0;">{ "Rust Calc Pro" }</h1>
            
            // Display
            <div style="background: #34495e; padding: 20px; border-radius: 10px; text-align: right; font-size: 2.5rem; margin-bottom: 20px; overflow: hidden;">
                { state.current_value }
            </div>

            <input ref={input_ref} type="number" placeholder="0" 
                style="width: 100%; padding: 15px; border-radius: 10px; border: none; font-size: 1.2rem; margin-bottom: 20px; box-sizing: border-box;" />

            // Operatoren Grid
            <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; margin-bottom: 20px;">
                { for vec!["+", "-", "*", "/"].into_iter().map(|op| {
                    let on_click = {
                        let send_op = send_op.clone();
                        move |_| send_op(Some(op))
                    };
                    html! {
                        <button onclick={on_click} style="padding: 20px; font-size: 1.5rem; border: none; border-radius: 10px; background: #f39c12; color: white; cursor: pointer;">{ op }</button>
                    }
                }) }
            </div>

            // Action Buttons
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 10px;">
                <button onclick={save_result} style="padding: 15px; border-radius: 10px; border: none; background: #27ae60; color: white; cursor: pointer;">{ "Speichern" }</button>
                <button onclick={reset} style="padding: 15px; border-radius: 10px; border: none; background: #c0392b; color: white; cursor: pointer;">{ "Reset" }</button>
            </div>

            // Historie
            if !state.history.is_empty() {
                <div style="margin-top: 25px; border-top: 1px solid #7f8c8d; padding-top: 15px;">
                    <h3 style="margin-bottom: 10px;">{ "Historie" }</h3>
                    <ul style="list-style: none; padding: 0; max-height: 100px; overflow-y: auto;">
                        { for state.history.iter().rev().map(|h| html! {
                            <li style="background: #34495e; margin-bottom: 5px; padding: 5px 10px; border-radius: 5px;">{ h }</li>
                        }) }
                    </ul>
                </div>
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}