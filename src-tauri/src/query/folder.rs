use diesel::prelude::*;
use serde::Serialize;

use crate::db::establish_connection;
use crate::db::models;
use crate::schema;

#[derive(Debug, Serialize)]
pub struct FolderQueryResult {
    list: Vec<models::Folder>,
}

pub fn get_folders() -> FolderQueryResult {
    let folder = schema::folders::dsl::folders;
    let connection = &mut establish_connection();

    let result = folder
        .load::<models::Folder>(connection)
        .expect("Error loading requests");

    FolderQueryResult { list: result }
}

pub fn create_new_folder() -> models::Folder {
    let folder = schema::folders::dsl::folders;
    let folder_id = schema::folders::dsl::id;
    let connection = &mut establish_connection();

    let new_folder = models::Folder {
        id: 0,
        name: "New Folder".to_string(),
        parent_id: None,
    };

    diesel::insert_into(folder)
        .values(&new_folder)
        .execute(connection)
        .expect("Error saving new folder");

    let result = folder
            .order(folder_id.desc())
            .first::<models::Folder>(connection)
            .unwrap();

    result
}
