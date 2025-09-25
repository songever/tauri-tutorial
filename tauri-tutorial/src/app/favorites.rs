
use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    let mut hovered_id = use_signal(|| None);
    // Create a pending resource that resolves to the list of dogs from the backend
    // Wait for the favorites list to resolve with `.suspend()`
    let mut list_dog = use_resource(super::list_dogs);
    let favorites = list_dog.suspend()?;

    rsx! { 
        div { id: "favorites",
            div {id: "favorites-container", 
                for (id, url) in favorites().unwrap() {
                    div { 
                        // Render a div for each photo using the dog's ID as the list key
                        key: "{id}",
                        class: "favorite-dog",
                        onmouseover: move |_| hovered_id.set(Some(id)), // 鼠标悬停时设置当前 ID
                        onmouseout: move |_| hovered_id.set(None),      // 鼠标移开时清除 ID
                        style: "position: relative;", // 使按钮定位到图片内部
                        img { 
                            src: "{url}",
                        }
                        // 如果当前悬停的 ID 是这个图片的 ID，则显示按钮
                        button { id: "delete",
                            onclick: move |_| async move {
                                tracing::info!("Deleting favorite with id: {}", id);
                                // 调用删除函数
                                super::delete_fav(id).await;
                                list_dog.restart();
                            },
                            style: "
                                position: absolute;
                                bottom: 10px;
                                left: 10px;
                                background-color: red;
                                color: white;
                                border: none;
                                padding: 5px 10px;
                                cursor: pointer;
                                border-radius: 5px;
                            ",
                            "Delete"
                        } 
                    }
                }
            }
        }
    }    
}