#![allow(non_snake_case)]
mod nav;
mod favorites;

use nav::*;
use favorites::*;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
static CSS: Asset = asset!("/assets/styles.css");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Deserialize)]
struct DogApi {
    message: String,
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,

    // #[route("/:..segments")]
    // PageNotFound { segments: Vec<String>}, 
}

#[derive(Serialize, Deserialize)]
struct SaveDogArgs {
    url: String,
}

async fn save_dog(url: String) {
    let js_args = to_value(&SaveDogArgs{ url }).unwrap();
    invoke("save_dog", js_args).await;
}

async fn list_dogs() -> Result<Vec<(usize, String)>, ()> {
    tracing::info!("listing dogs");
    let result = invoke("list_dogs", JsValue::NULL).await;
    let dogs: Vec<(usize, String)> = serde_wasm_bindgen::from_value(result)
        .map_err(|e| tracing::error!("{}",e.to_string()))?;
    Ok(dogs)
}

async fn delete_fav(id: usize) {
    tracing::info!("delete the id: {}", id);
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({ "id": id })).unwrap();
    invoke("delete_fav", args).await;
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
        // DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "Hotdog! 🌭"}
        }
    }
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            img {
                src: img_src.cloned().unwrap_or_default(),
                style: "max-height: 300px;"
            },

        }
        div { id: "buttons",
            button { id: "skip",
                onclick: move |_| img_src.restart(),
                "Skip"
            }
            button { id: "save",
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    save_dog(current).await;
                },
                "Save!"
            }
        }
    }
}
