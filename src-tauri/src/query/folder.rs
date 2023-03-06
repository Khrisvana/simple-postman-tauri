use diesel::alias;
use diesel::prelude::*;
use serde::Serialize;

use crate::db::establish_connection;
use crate::db::models::Folder;
use crate::schema::folders;

#[derive(Debug, Serialize)]
pub struct FolderQueryResult {
    list: Vec<Folder>,
}

pub fn create_new_folder() -> Folder {
    let folder = folders::dsl::folders;
    let folder_id = folders::dsl::id;
    let connection = &mut establish_connection();

    let new_folder = Folder {
        id: 0,
        name: "New Folder".to_string(),
        parent_id: None,
        order_number: 1
    };

    diesel::insert_into(folder)
        .values(&new_folder)
        .execute(connection)
        .expect("Error saving new folder");

    let result = folder
        .order(folder_id.desc())
        .first::<Folder>(connection)
        .unwrap();

    result
}
