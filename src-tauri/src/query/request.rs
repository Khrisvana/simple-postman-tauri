use diesel::prelude::*;
use serde::{Serialize};

use crate::db::establish_connection;
use crate::db::models; 
use crate::schema; 

pub struct Request{}
 
#[derive(Debug, Serialize)]
pub struct RequestQueryResult {
    list: Vec<models::Request>
}


impl Request {
    pub fn get_requests() -> RequestQueryResult {
        use schema::request::dsl::*;

        let connection = &mut establish_connection();
        // let query = schema::request::dsl::request.table();
    
        let result = request.load::<models::Request>(connection).expect("Error loading requests");
    
        RequestQueryResult { list: result }
    }
}