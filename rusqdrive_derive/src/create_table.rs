// * Struct to mount the `CREATE TABLE` query

#[derive(Debug, Default, Clone)]
pub struct CreateTable {
    pub col: String,
    pub typ: String,
    pub not_null: String,
    pub unique: String,
}
