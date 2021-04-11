// * CONNECTION
pub fn connect_db(
    tablename: &str,
    max_threads: u32,
) -> Result<super::prelude::Pool, r2d2::Error> {
    // const DB_FILE: &str = "db.sqlite3";
    // const MAX_THREADS: u32 = 1;

    let manager = r2d2_sqlite::SqliteConnectionManager::file(tablename);
    match r2d2::Pool::builder().max_size(max_threads).build(manager) {
        Ok(connection_pool) => Ok(connection_pool),
        Err(error) => Err(error),
    }
}
