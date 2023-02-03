use diesel::prelude::*;
use serde::Serialize;
use crate::schema::requests;

#[derive(Queryable, Insertable, Serialize, Debug)] 
#[diesel(table_name = requests)]
pub struct Request {
    pub id: i32,
    pub name: Option<String>,
    pub url: Option<String>,
    pub method: String,
}