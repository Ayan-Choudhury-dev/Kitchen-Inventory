// use serde::{Deserialize, Serialize};
// use tauri::State;
use tauri_plugin_sql::{Migration, MigrationKind};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct User {
//     id: i64,
//     name: String,
//     email: String,
// }

// #[tauri::command]
// async fn add_user(
//     pool: State<'_, SqlitePool>,
//     name: String,
//     email: String,
// ) -> Result<(), String> {
//     let query = "INSERT INTO users (name, email) VALUES (?, ?)";
//     pool.execute(query, [name, email])
//         .await
//         .map_err(|e| e.to_string())?;
//     Ok(())
// }

// #[tauri::command]
// async fn get_users(pool: State<'_, SqlitePool>) -> Result<Vec<User>, String> {
//     let query = "SELECT id, name, email FROM users";
//     pool.select(query, [])
//         .await
//         .map_err(|e| e.to_string())
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create users table",
        sql: "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT
        )",
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default()
            .add_migrations("sqlite:users.db", migrations)
            .build())
        .plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}