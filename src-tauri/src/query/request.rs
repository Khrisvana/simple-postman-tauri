use diesel::prelude::*;
use serde::Serialize;
use std::fmt;

use tauri::{command};
use crate::db::establish_connection;
use crate::db::models::{NewRequest, Request};
use crate::schema::requests::{self, order_number, parent_id};

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

#[command]
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

#[command]
pub fn update_order(parent: Option<i32>, target_id: i32, index: i32) -> String {
    let connection = &mut establish_connection();

    diesel::update(requests::table.find(target_id))
        .set((parent_id.eq(parent), order_number.eq(index)))
        .execute(connection)
        .expect("unable to update order");

    let other_child = requests::table
        .filter(order_number.ge(index))
        .filter(parent_id.eq(parent))
        .filter(requests::id.ne(target_id))
        .load::<Request>(connection)
        .expect("cant get other child");

    let mut target_index = index;
    for other in other_child.into_iter() {
        target_index += 1;
        
        diesel::update(requests::table.find(&other.id))
            .set(order_number.eq(&target_index))
            .execute(connection)
            .expect("unable to update order");
    }

    "order updated".to_string()
}

#[command]
pub fn get_request(id: i32) -> Request {
    let connection = &mut establish_connection();

    let request = requests::table
        .find(id)
        .first::<Request>(connection)
        .expect("Error loading requests");

    request
}

#[command]
pub fn store_request(parent: Option<i32>, order: i32, is_folder: bool) -> Request {
    let request = requests::dsl::requests;
    let request_id = requests::dsl::id;

    let connection = &mut establish_connection();
    let mut method: Option<String> = Some(Methods::GET.to_string());
    let mut name: String = "New Request".to_string();

    if is_folder {
        method = None;
        name = "New Folder".to_string();
    }
    
    let new_request = NewRequest {
        method: method,
        name: name,
        url: None,
        order_number: order,
        parent_id: parent
    };

    diesel::insert_into(request)
        .values(&new_request)
        .execute(connection)
        .expect("Error saving request");

    let result = request
        .order(request_id.desc())
        .first::<Request>(connection)
        .unwrap();

    result
}
