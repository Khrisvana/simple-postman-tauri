use diesel::prelude::*;

#[derive(Queryable)]
pub struct Request {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub method: String,
}