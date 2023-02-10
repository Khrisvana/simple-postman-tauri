use diesel::prelude::*;
use serde::Serialize;
use std::fmt;

use crate::db::establish_connection;
use crate::db::models::{Folder, NewRequest, Request};
use crate::schema::folders::parent_id;
use crate::schema::{requests, folders};

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

#[derive(Serialize)]
pub struct RequestResult {
    #[serde(flatten)]
    pub fd: Folder,
    pub result: Vec<Request>,
}

#[derive(Serialize, Clone, Debug)]
pub struct MappedResult {
    #[serde(flatten)]
    pub fd: Folder,
    pub request: Option<Vec<Request>>,
    pub items: Option<Vec<MappedResult>>,
}

pub fn map_requests() -> Vec<MappedResult> {
    let connection = &mut establish_connection();
    let fd = folders::table
        .filter(parent_id.is_null())
        .load::<Folder>(connection)
        .unwrap();

    let mut mapped_result: Vec<MappedResult> = vec![];
    for item in fd.into_iter() {
        mapped_result.push(recurse(&item, connection));
    }

    mapped_result
}

fn recurse(parent: &Folder, connection: &mut SqliteConnection) -> MappedResult {
    let filtered_child = folders::table
        .filter(parent_id.eq(parent.id))
        .load::<Folder>(connection)
        .unwrap();
    let requests = Request::belonging_to(&parent)
        .load::<Request>(connection)
        .expect("Error loading requests");

    let mut parent_temp: MappedResult = MappedResult {
        fd: parent.clone(),
        request: Some(requests),
        items: None,
    };

    let mut childrens: Vec<MappedResult> = vec![];

    for each_child in filtered_child.into_iter() {
        childrens.push(recurse(&each_child, connection));
    }
    parent_temp.items = Some(childrens);

    parent_temp
}

pub fn get_request(id: i32) -> Request {
    let connection = &mut establish_connection();

    let request = requests::table.find(id)
        .first::<Request>(connection)
        .expect("Error loading requests");

    request
}

pub fn store_request() -> Request {
    let request = requests::dsl::requests;
    let request_id = requests::dsl::id;

    let connection = &mut establish_connection();

    let new_request = NewRequest {
        method: Methods::GET.to_string(),
        name: Some("New Request".to_string()),
        url: None,
        folder_id: None,
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
