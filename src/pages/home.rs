use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="bg-gray-800 p-2 m-2 rounded-lg">
            <h1 class="text-4xl font-bold text-white">Welcome to the Cruces Chess Club!</h1>
        </div>
        <div>
        </div>
    }
}
