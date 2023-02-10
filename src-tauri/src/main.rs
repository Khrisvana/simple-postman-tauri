#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate diesel;
extern crate diesel_migrations;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

mod db;
mod query;
mod schema;
mod cmd;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn main() {
    let mut connection = db::establish_connection();

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error migrating");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmd::get_request,
            cmd::create_new_request,
            cmd::get_folders,
            cmd::create_new_folder,
            cmd::get_folders_with_child,
            cmd::map_requests,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
