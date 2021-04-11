pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type DbConnection =
    r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub use rusqdrive_derive::Rusqdrive;
pub use rusqlite::{params, Result as RusqliteResult};
