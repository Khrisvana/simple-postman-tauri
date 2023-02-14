use tauri::{command}; 
use crate::query;
use crate::db::models;

#[command]
pub fn get_request(id: i32) -> models::Request {
    let result = query::request::get_request(id);

    result
}

#[command]
pub fn request_list() -> Vec<query::request::MappedResult> {
    let result = query::request::request_list();

    result
}

#[command]
pub fn update_order(parent: Option<i32>, target_id: i32, index: i32) -> String {
    let result = query::request::update_order(parent, target_id, index);

    result
}

// #[command]
// pub fn create_new_request() -> models::Request {
//     let result = query::request::store_request();

//     result
// }

#[command]
pub fn get_folders() -> query::folder::FolderQueryResult {
    let result = query::folder::get_folders();

    result
}

#[command]
pub fn get_folders_with_child() -> Vec<(models::Folder, models::Folder)> {
    let result = query::folder::get_folder_childs();

    result
}

#[command]
pub fn create_new_folder() -> models::Folder {
    let result = query::folder::create_new_folder();

    result
}