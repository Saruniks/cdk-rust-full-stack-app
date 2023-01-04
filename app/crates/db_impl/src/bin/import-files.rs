use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Read,
};

use db_common::establish_connection;
use db_impl::schema::{first_group_member, second_group_member};
use diesel::prelude::*;

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();

    let paths = fs::read_dir("/home/test/full-stack-proj/files").unwrap();

    let mut count = 0;
    for path in paths {
        if path.as_ref().unwrap().path().extension() == Some(OsStr::new("webp")) {
            let mut file = File::open(path.as_ref().unwrap().path()).unwrap();
            let mut data = Vec::new();

            file.read_to_end(&mut data).unwrap();

            let result = if count % 2 == 0 {
                diesel::insert_into(first_group_member::table)
                    .values((
                        first_group_member::title.eq(path.unwrap().path().file_name().unwrap().to_str().unwrap()),
                        first_group_member::file_blob.eq(data),
                    ))
                    .on_conflict_do_nothing()
                    .execute(&conn)
            } else {
                diesel::insert_into(second_group_member::table)
                    .values((
                        second_group_member::title.eq(path.unwrap().path().file_name().unwrap().to_str().unwrap()),
                        second_group_member::file_blob.eq(data),
                    ))
                    .on_conflict_do_nothing()
                    .execute(&conn)
            };

            if let Ok(res) = result {
                if res > 0 {
                    count += 1;
                }
            }
        }
    }
    println!("Imported files count = {}", count);
}
