use crate::column_definition::Column;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TpdEntry {
    pub tpd_size: usize,
    pub table_name: String,
    pub num_columns: usize,
    pub columns: Vec<Column>,
    pub tpd_flags: u32,
}

impl TpdEntry {
    pub fn new(table_name: &str, columns: Vec<Column>) -> Self {
        // Calculate the total size of the TpdEntry in bytes
        // The size includes:
        // 4 bytes for `tpd_size` field
        // `table_name.len()` bytes for the table name (dynamic size)
        // 4 bytes for `num_columns` field
        // 4 bytes for `cd_offset` field
        // 4 bytes for `tpd_flags` field
        // `columns.len() * 36` bytes for column descriptors (each `cd_entry` is 36 bytes)
        let tpd_size = 4 + table_name.len() + 4 + 4 + 4 + columns.len() * 36;
        TpdEntry {
            tpd_size,
            table_name: table_name.to_string(),
            num_columns: columns.len(),
            columns,
            tpd_flags: 0, // Default flags
        }
    }
}

#[derive(Debug)]
pub struct TpdList {
    pub list_size: usize,
    pub num_tables: usize,
    pub tpd_entries: HashMap<String, TpdEntry>, // Use HashMap for fast lookup
}

impl TpdList {
    pub fn new() -> Self {
        TpdList {
            list_size: 0,
            num_tables: 0,
            tpd_entries: HashMap::new(),
        }
    }

    pub fn add_tpd_entry(&mut self, entry: TpdEntry) -> Result<(), String> {
        if self.tpd_entries.contains_key(&entry.table_name) {
            return Err("Duplicate table name".to_string());
        }
        self.list_size += entry.tpd_size;
        self.num_tables += 1;
        self.tpd_entries.insert(entry.table_name.clone(), entry);
        Ok(())
    }

    pub fn drop_tpd_entry(&mut self, table_name: &str) -> Result<(), String> {
        if self.tpd_entries.remove(table_name).is_none() {
            return Err("Table does not exist".to_string());
        }
        self.num_tables -= 1;
        Ok(())
    }

    pub fn get_tpd_entry(&self, table_name: &str) -> Option<&TpdEntry> {
        self.tpd_entries.get(table_name)
    }
}
