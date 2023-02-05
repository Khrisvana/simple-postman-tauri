use diesel::prelude::*;
use serde::Serialize;
use std::fmt;

use crate::db::establish_connection;
use crate::db::models::{Request, NewRequest, Folder};
use crate::schema::{self, folders};


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
pub struct UserAPI {
    #[serde(flatten)]
    pub fd: Folder,

    pub result: Request,
}

pub fn get_requests() -> Vec<(Folder, Vec<Request>)> {
    let connection = &mut establish_connection();
    // let query = schema::request::dsl::request.table();

    let fd = folders::table
        .load::<Folder>(connection)
        .unwrap();
        
    let result = Request::belonging_to(&fd)
    .load::<Request>(connection)
    .expect("Error loading requests")
    .grouped_by(&fd);

    let data: Vec<_> = fd.into_iter().zip(result).collect();

    // let result = requests
    //     .load::<Request>(connection)
    //     .expect("Error loading requests");

    // let fdres: Vec<UserAPI> = data
    //             .into_iter()
    //             .map(|(fd, result)| {
    
    //                 UserAPI {
    //                     fd,
    //                     result,
    //                 }
    //             })
    //             .collect();


    data
    // RequestQueryResult { list: result }
}

pub fn store_request() -> Request {
    let request = schema::requests::dsl::requests;
    let request_id = schema::requests::dsl::id;

    let connection = &mut establish_connection();

    let new_request = NewRequest {
        method: Methods::GET.to_string(),
        name: Some("New Request".to_string()),
        url: None,
        folder_id: None
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
