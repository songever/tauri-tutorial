// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions,Pool, Sqlite};
use tauri::{App, Manager};

type Db = Pool<Sqlite>;
struct AppState {
    db: Db,
}
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn save_dog(state: tauri::State<'_, AppState>, url: String) -> Result<(), String> {
    let db = &state.db;
    
    sqlx::query("INSERT INTO dogs (url) VALUES (?)")
        .bind(url)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn setup_db(app: &App) -> Db {
    let mut path = app.path().app_data_dir().expect("failed to get data_dir");

    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => {}
        Err(err) => {
            panic!("error creating directory {}", err);
        }
    };

    path.push("hotdog.sqlite");
    println!("{:?}", path);
    Sqlite::create_database(
        format!(
            "sqlite:{}",
            path.to_str().expect("path should be something")
        )
        .as_str(),
    )
    .await
    .expect("failed to create database");

    let db = SqlitePoolOptions::new()
        .connect(path.to_str().unwrap())
        .await
        .unwrap();

    sqlx::migrate!("../migrations").run(&db).await.unwrap();

    db
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![save_dog])
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let db = setup_db(&app).await;

                app.manage(AppState{ db });
                Ok(())
            })
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
