use tauri::{command}; 
use crate::query;

#[command]
pub fn get_requests() -> query::request::RequestQueryResult {
    let result = query::request::Request::get_requests();

    result
}