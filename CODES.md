# Codes

## CRUD rusqlite

```rs

// *  CREATE TABLE
pub fn create_table(conn: &Connection) -> RusqliteResult<()> {
    conn.execute(
        "CREATE TABLE users (
            id	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            name	TEXT NOT NULL,
            age	INTEGER NOT NULL,
            alive	BOOLEAN NOT NULL
        );",
        NO_PARAMS,
    )?;

    Ok(())
}

// *  READ COUNT
pub fn read_count(conn: &Connection) -> RusqliteResult<i64> {
    // *  READ COUNT
    let query_count = "SELECT COUNT(ALL) FROM users";

    let count: RusqliteResult<i64> =
        conn.query_row(query_count, NO_PARAMS, |r| r.get(0));

    count
}

// * READ ALL
pub fn read_all(conn: &Connection) -> RusqliteResult<Vec<User>> {
    let mut stmt = conn.prepare("SELECT * FROM users")?;
    let data_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
            alive: row.get(3)?,
        })
    })?;

    let mut data_vec = vec![];

    for data in data_iter {
        data_vec.push(data?);
    }

    Ok(data_vec)
}

// * INSERT ONE
pub fn insert_one(conn: &Connection, data: &User) -> RusqliteResult<String> {
    match conn.execute(
        "INSERT INTO users (name, age, alive) VALUES (?1, ?2, ?3)",
        params![data.name, data.age, data.alive],
    ) {
        Ok(_) => Ok("CREATED".into()),
        Err(error) => Err(error),
    }
}

// * UPDATE ONE
pub fn update_one(
    conn: &Connection,
    data: &User,
    id: i32,
) -> RusqliteResult<String> {
    let query = "UPDATE users SET name=?1, age=?2, alive=?3 WHERE id=?4";

    match conn.execute(query, params![data.name, data.age, data.alive, id]) {
        Ok(_) => Ok("UPDATED".into()),
        Err(error) => Err(error),
    }
}

// * DELETE ONE
pub fn delete_one(conn: &Connection, id: i32) -> RusqliteResult<String> {
    let query = "DELETE FROM users WHERE id=?1";
    match conn.execute(query, params![id]) {
        Ok(_) => Ok("DELETED".into()),
        Err(error) => Err(error),
    }
}

```

## Traits

```rs

use rusqlite::{Connection, Result as RusqliteResult};

// * CONNECTION
pub fn connect() -> RusqliteResult<Connection> {
    Connection::open_in_memory()
}

pub trait Database {
    type Tablename;

    fn create_table(&self, conn: &Connection) -> RusqliteResult<()>;
    fn read_count(&self, conn: &Connection) -> RusqliteResult<i64>;
    fn read_all(
        &self,
        conn: &Connection,
    ) -> RusqliteResult<Vec<Self::Tablename>>;
    fn insert_one(
        &self,
        conn: &Connection,
        data: &Self::Tablename,
    ) -> RusqliteResult<String>;
    fn update_one(
        &self,
        conn: &Connection,
        data: &Self::Tablename,
        id: i32,
    ) -> RusqliteResult<String>;
    fn delete_one(&self, conn: &Connection, id: i32) -> RusqliteResult<String>;
}

```

## Pluralize tablename 

```rs

fn pluralize_tablename(mut tablename: String) -> String {
    tablename.push_str("s");
    tablename.into()
}

```

## Issue: proc-macro derive panicked message: not yet implemented: Option<String> - Type not supported

```rs


use syn::{GenericArgument, PathArguments, Type};

fn extract_type_from_option(ty: &Type) -> Type {
    fn path_is_option(path: &Path) -> bool {
        leading_colon.is_none()
            && path.segments.len() == 1
            && path.segments.iter().next().unwrap().ident == "Option"
    }

    match ty {
        Type::Path(typepath)
            if typepath.qself.is_none() && path_is_option(typepath.path) =>
        {
            // Get the first segment of the path (there is only one, in fact: "Option"):
            let type_params =
                typepath.path.segments.iter().first().unwrap().arguments;
            // It should have only on angle-bracketed param ("<String>"):
            let generic_arg = match type_params {
                PathArguments::AngleBracketed(params) => {
                    params.args.iter().first().unwrap()
                }
                _ => panic!("TODO: error handling"),
            };
            // This argument must be a type:
            match generic_arg {
                GenericArgument::Type(ty) => ty,
                _ => panic!("TODO: error handling"),
            }
        }
        _ => panic!("TODO: error handling"),
    }
}

```