#[derive(Debug)]
pub enum ColumnType {
    Int,
    Varchar(usize),
    Char(usize),
}

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub col_type: ColumnType,
    pub not_null: bool,
}

impl Column {
    pub fn new(name: &str, col_type: ColumnType, not_null: bool) -> Self {
        Column {
            name: name.to_string(),
            col_type,
            not_null,
        }
    }
}
