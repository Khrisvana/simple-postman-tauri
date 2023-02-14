use diesel::prelude::*;
use serde::{Serialize, Serializer};
use std::fmt;

use crate::db::establish_connection;
use crate::db::models::{Folder, NewRequest, Request};
use crate::schema::requests;

#[derive(Debug, Serialize)]
pub struct RequestQueryResult {
    list: Vec<Request>,
}

#[derive(Debug)]
enum Methods {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl fmt::Display for Methods {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, PartialOrd)]
pub struct MappedResult {
    #[serde(flatten)]
    pub fd: Request,
    pub items: Option<Vec<MappedResult>>,
}

pub fn request_list() -> Vec<MappedResult> {
    let connection = &mut establish_connection();
    let first_folder: Vec<Request> = requests::table
        .filter(requests::parent_id.is_null())
        .order_by(requests::order_number.asc())
        .load::<Request>(connection)
        .unwrap();

    let mut mapped_result: Vec<MappedResult> = vec![];

    for item in first_folder.into_iter() {
        mapped_result.push(add_child(&item, connection));
    }

    mapped_result
}

fn add_child(parent: &Request, connection: &mut SqliteConnection) -> MappedResult {
    let filtered_child = requests::table
        .filter(requests::parent_id.eq(parent.id))
        .order_by(requests::order_number.asc())
        .load::<Request>(connection)
        .unwrap();

    let mut parent_temp: MappedResult = MappedResult {
        fd: parent.clone(),
        items: None,
    };

    let mut childrens: Vec<MappedResult> = vec![];

    for each_child in filtered_child.into_iter() {
        childrens.push(add_child(&each_child, connection));
    }
    parent_temp.items = Some(childrens);

    parent_temp
}

pub fn update_order() {
    
}

pub fn get_request(id: i32) -> Request {
    let connection = &mut establish_connection();

    let request = requests::table
        .find(id)
        .first::<Request>(connection)
        .expect("Error loading requests");

    request
}

// pub fn store_request() -> Request {
//     let request = requests::dsl::requests;
//     let request_id = requests::dsl::id;

//     let connection = &mut establish_connection();

//     let new_request = NewRequest {
//         method: Methods::GET.to_string(),
//         name: Some("New Request".to_string()),
//         url: None,
//         folder_id: None,
//         order_number: 1,
//     };

//     diesel::insert_into(request)
//         .values(&new_request)
//         .execute(connection)
//         .expect("Error saving request");

//     let result = request
//         .order(request_id.desc())
//         .first::<Request>(connection)
//         .unwrap();

//     result
// }
