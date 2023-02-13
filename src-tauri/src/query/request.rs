use diesel::prelude::*;
use serde::{Serialize, Serializer};
use std::fmt;

use crate::db::establish_connection;
use crate::db::models::{Folder, NewRequest, Request};
use crate::schema::folders::parent_id;
use crate::schema::requests::folder_id;
use crate::schema::{folders, requests};

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
    pub fd: Folder,
    // #[serde(flatten)]
    // pub rq: Option<Request>,
    pub request: Option<Vec<Request>>,
    pub items: Option<Vec<MappedResult>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum MappedOption {
    Request(Request),
    Folder(Folder),
}

impl Serialize for MappedOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MappedOption::Request(value) => value.serialize(serializer),
            MappedOption::Folder(value) => value.serialize(serializer),
        }
    }
}

pub fn map_requests() -> Vec<MappedResult> {
    let connection = &mut establish_connection();
    let first_folder: Vec<Folder> = folders::table
        .filter(parent_id.is_null())
        .load::<Folder>(connection)
        .unwrap();

    // let first_request: Vec<Request> = requests::table
    //     .filter(folder_id.is_null())
    //     .load::<Request>(connection)
    //     .unwrap();

    let mut mapped_result: Vec<MappedResult> = vec![];

    // for item in first_request.into_iter() {
    //     mapped_result.push(MappedResult {
    //         fd: None,
    //         rq: Some(item.clone()),
    //         request: None,
    //         items: None,
    //     });
    // }

    for item in first_folder.into_iter() {
        mapped_result.push(recurse(&item, connection));
    }

    mapped_result
}

fn recurse(parent: &Folder, connection: &mut SqliteConnection) -> MappedResult {
    let filtered_child = folders::table
        .filter(parent_id.eq(parent.id))
        .order_by(folders::order_number.asc())
        .load::<Folder>(connection)
        .unwrap();

    let requests = Request::belonging_to(&parent)
        .order_by(requests::order_number.asc())
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

    let request = requests::table
        .find(id)
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
        order_number: 1,
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
