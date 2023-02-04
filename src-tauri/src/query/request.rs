use diesel::prelude::*;
use serde::Serialize;
use std::fmt;

use crate::db::establish_connection;
use crate::db::models;
use crate::schema;


#[derive(Debug, Serialize)]
pub struct RequestQueryResult {
    list: Vec<models::Request>,
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

pub fn get_requests() -> RequestQueryResult {
    use schema::requests::dsl::*;

    let connection = &mut establish_connection();
    // let query = schema::request::dsl::request.table();

    let result = requests
        .load::<models::Request>(connection)
        .expect("Error loading requests");

    RequestQueryResult { list: result }
}

pub fn store_request() -> models::Request {
    let request = schema::requests::dsl::requests;
    let request_id = schema::requests::dsl::id;

    let connection = &mut establish_connection();

    let new_request = models::Request {
        id: 0,
        method: Methods::GET.to_string(),
        name: Some("New Request".to_string()),
        url: None,
    };

    diesel::insert_into(request)
        .values(&new_request)
        .execute(connection)
        .expect("Error saving request");

    let result = request
        .order(request_id.desc())
        .first::<models::Request>(connection)
        .unwrap();

    result 
}
