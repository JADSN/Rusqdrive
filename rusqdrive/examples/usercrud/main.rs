mod models;

use std::error::Error;

use crate::models::{
    constants::{MAX_THREADS, TABLENAME},
    user::User,
};

use rusqdrive::database;

use chrono::{Duration, Utc};

fn run() -> Result<(), Box<dyn Error>> {
    let now = Utc::now();

    let plus_1_day = now + Duration::days(1);

    let conn = database::connect_db(TABLENAME, MAX_THREADS)?.get()?;
    let user = User::default();

    user.create_table(&conn)?;

    // *  READ COUNT
    let count = user.read_count(&conn)?;
    println!("COUNT: {}", &count);

    // *  READ ALL
    let all_users = user.read_all(&conn)?;
    println!("ALL: {:?}", all_users);

    // * INSERT ONE
    let data = User::new(None, Some("Labolicha".into()), 23, true, now, None);

    let status = user.insert_one(&conn, &data)?;
    println!("STATUS: {}", status);

    let status = user.insert_one(&conn, &data)?;
    println!("STATUS: {}", status);

    // *  READ COUNT
    let count = user.read_count(&conn)?;
    println!("COUNT: {}", &count);

    // *  READ ALL
    let all = user.read_all(&conn)?;
    println!("ALL: {:?}", all);

    // * UPDATE ONE
    let new_data = User::new(
        None,
        Some("UPDATED".into()),
        100,
        false,
        now,
        Some(plus_1_day),
    );
    let status = user.update_one(&conn, &new_data, 1)?;
    println!("STATUS: {}", status);

    // *  READ ALL
    let all = user.read_all(&conn)?;
    println!("ALL: {:?}", all);

    // * DELETE ONE
    let status = user.delete_one(&conn, 1)?;
    println!("STATUS: {}", status);

    // *  READ COUNT
    let count = user.read_count(&conn)?;
    println!("COUNT: {}", &count);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
