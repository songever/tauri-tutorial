#![allow(non_snake_case)]

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

#[derive(Serialize, Deserialize)]
struct SaveDogArgs {
    image: String,
}

async fn save_dog(image: String) -> Result<(), JsValue> {
    let js_args = to_value(&SaveDogArgs{ image }).unwrap();
    invoke("save_dog", js_args).await;
    Ok(())
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
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

    // let save_dog = move |url| async move {
    //     let js_args = to_value(&serde_json::json!({ "image": url })).unwrap();
    //         invoke("save_dog", js_args).await;
    // };

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
                    _ = save_dog(current).await;
                },
                "Save!"
            }
        }
    }
}
