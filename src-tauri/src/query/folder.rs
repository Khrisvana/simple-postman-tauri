use diesel::alias;
use diesel::prelude::*;
use serde::Serialize;

use crate::db::establish_connection;
use crate::db::models::{Folder};
use crate::schema::folders::{self};

#[derive(Debug, Serialize)]
pub struct FolderQueryResult {
    list: Vec<Folder>,
}

pub fn get_folders() -> FolderQueryResult {
    let folder = folders::dsl::folders;
    let connection = &mut establish_connection();

    let result = folder
        .load::<Folder>(connection)
        .expect("Error loading requests");

    FolderQueryResult { list: result }
}

pub fn get_folder_childs() -> Vec<(Folder, Folder)> {
    let folder_alias = alias!(folders as the_folder);

    let connection = &mut establish_connection();

    // let parent = folders::table
    //     .filter(parent_id.eq(None))
    //     .get_results::<Folder>(connection); 

    let result = folders::table
        .inner_join(folder_alias.on(folders::id.eq(folder_alias.field(folders::parent_id).assume_not_null())))
        .select((folders::all_columns, folder_alias.fields(folders::all_columns)))
        .load::<(Folder, Folder)>(connection)
        .unwrap();
 
    // for (folder, parent) in parents {

    // }
    

    result
} 

pub fn create_new_folder() -> Folder {
    let folder = folders::dsl::folders;
    let folder_id = folders::dsl::id;
    let connection = &mut establish_connection();

    let new_folder = Folder {
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
        .first::<Folder>(connection)
        .unwrap();

    result
}
