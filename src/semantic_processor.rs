use crate::column_definition::Column;
use crate::table_manager::{TpdEntry, TpdList};
use crate::token::{Token, TokenType};

pub struct SemanticProcessor<'a> {
    tpd_list: &'a mut TpdList,
}

impl<'a> SemanticProcessor<'a> {
    pub fn new(tpd_list: &'a mut TpdList) -> Self {
        SemanticProcessor { tpd_list }
    }

    pub fn process_create_table(&mut self, table_name: &str, columns: Vec<Column>) -> Result<(), String> {
        let entry = TpdEntry::new(table_name, columns);
        self.tpd_list.add_tpd_entry(entry)
    }

    pub fn process_drop_table(&mut self, table_name: &str) -> Result<(), String> {
        self.tpd_list.drop_tpd_entry(table_name)
    }

    pub fn process_list_tables(&self) {
        if self.tpd_list.num_tables == 0 {
            println!("No tables defined.");
        } else {
            for (table_name, _) in &self.tpd_list.tpd_entries {
                println!("{}", table_name);
            }
        }
    }

    pub fn process_list_schema(&self, table_name: &str) {
        if let Some(entry) = self.tpd_list.get_tpd_entry(table_name) {
            println!("Schema for table '{}':", entry.table_name);
            for column in &entry.columns {
                println!("{:?}", column);
            }
        } else {
            println!("Table '{}' does not exist.", table_name);
        }
    }
}

