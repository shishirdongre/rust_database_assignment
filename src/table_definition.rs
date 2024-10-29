use crate::column_definition::Column;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Table {
    pub fn new(name: &str, columns: Vec<Column>) -> Self {
        Table {
            name: name.to_string(),
            columns,
        }
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }
}
