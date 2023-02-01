use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::establish_connection;
use crate::models; 
 
pub fn get_request() -> Vec<Request> {
    let connection = establish_connection();

    
}