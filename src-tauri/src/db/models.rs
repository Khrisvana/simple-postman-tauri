use diesel::{prelude::*};
use serde::Serialize;
use crate::schema::requests;
use crate::schema::folders;

#[derive(Queryable, Insertable, Serialize, Debug)] 
#[diesel(table_name = folders)]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
}

#[derive(Queryable, Insertable, Serialize, Debug)] 
#[diesel(table_name = requests)]
pub struct Request {
    pub id: i32,
    pub name: Option<String>,
    pub url: Option<String>,
    pub method: String,
}