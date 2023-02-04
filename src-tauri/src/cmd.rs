use tauri::{command}; 
use crate::query;
use crate::db::models;

#[command]
pub fn get_requests() -> query::request::RequestQueryResult {
    let result = query::request::get_requests();

    result
}

#[command]
pub fn create_new_request() -> models::Request {
    let result = query::request::store_request();

    result
}

#[command]
pub fn get_folders() -> query::folder::FolderQueryResult {
    let result = query::folder::get_folders();

    result
}

#[command]
pub fn create_new_folder() -> models::Folder {
    let result = query::folder::create_new_folder();

    result
}