// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_dog(image: String) -> Result<(), String> {
    use std::io::Write;
    println!("Saving dog image: {}", image); // 添加日志

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .map_err(|e| e.to_string())?;

    let _ = file.write_fmt(format_args!("{image}\n"));

    println!("Dog image saved successfully!"); // 添加日志
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![save_dog])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
