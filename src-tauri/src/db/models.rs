use diesel::prelude::*;
use serde::Serialize; 

use crate::schema::requests;
use crate::schema::folders;

#[derive(Identifiable, Queryable, Insertable, PartialEq, PartialOrd, Debug, Serialize, Clone)] 
#[diesel(table_name = folders)]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub order_number: i32,
    pub parent_id: Option<i32>,
} 

// #[derive(Debug, Identifiable, Associations, Queryable, PartialEq, Serialize)]
// #[diesel(belongs_to(Folder, foreign_key = parent_id))]
// #[diesel(table_name = folders)]
// pub struct ChildFolder {
//     pub id: i32,
//     pub name: String,
//     pub parent_id: Option<i32>,
// }

#[derive(Identifiable, Queryable, PartialEq, PartialOrd, Debug, Clone, Serialize)]
#[diesel(table_name = requests)]
pub struct Request {
    pub id: i32,
    pub name: String,
    pub url: Option<String>,
    pub method: Option<String>,
    pub order_number: i32,
    pub parent_id: Option<i32>
}

#[derive( Insertable,  Debug)] 
#[diesel(table_name = requests)]
pub struct NewRequest {
    pub name: String,
    pub url: Option<String>,
    pub method: Option<String>,
    pub order_number: i32,
    pub parent_id: Option<i32>
}