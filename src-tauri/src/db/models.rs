use diesel::prelude::*;
use serde::Serialize; 

use crate::schema::requests;
use crate::schema::folders;

#[derive(Identifiable, Queryable, Insertable, PartialEq, Debug, Serialize, Clone)] 
#[diesel(table_name = folders)]
pub struct Folder {
    pub id: i32,
    pub name: String,
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

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize)] 
#[diesel(belongs_to(Folder))]
#[diesel(table_name = requests)]
pub struct Request {
    pub id: i32,
    pub name: Option<String>,
    pub url: Option<String>,
    pub method: String,
    pub folder_id: Option<i32>
}

#[derive( Insertable,  Debug)] 
#[diesel(table_name = requests)]
pub struct NewRequest {
    pub name: Option<String>,
    pub url: Option<String>,
    pub method: String,
    pub folder_id: Option<i32>
}