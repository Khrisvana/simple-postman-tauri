use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize, Debug)]
pub struct Request {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub method: String,
}