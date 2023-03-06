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

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn main() {
    let mut connection = db::establish_connection();

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error migrating");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            query::request::get_request,
            query::request::update_order,
            query::request::store_request,
            query::request::request_list,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
