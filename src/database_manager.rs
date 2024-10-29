use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

// use crate::generated_types::{ColumnDefinition, ColumnType as ProtoColumnType, Database, TableDefinition};
use crate::generated_types::generated_types::{
    ColumnDefinition, ColumnType as ProtoColumnType, Database, TableDefinition
};
use prost::Message;

use crate::nom_parser::{Column, ColumnType, Command};

// Conversion from Column to ColumnDefinition for protobuf
impl From<Column> for ColumnDefinition {
    fn from(column: Column) -> Self {
        ColumnDefinition {
            name: column.name,
            col_type: match column.col_type {
                ColumnType::INT => ProtoColumnType::Int as i32,
                ColumnType::STRING(_) => ProtoColumnType::String as i32,
            },
            length: match column.col_type {
                ColumnType::STRING(len) => len as u32,
                _ => 0,
            },
            not_null: column.not_null,
        }
    }
}

// DatabaseManager struct to handle file operations
pub struct DatabaseManager {
    file_path: String,
}

impl DatabaseManager {
    // Constructor
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    // Load the database from file or initialize if not present
    pub fn load_database(&self) -> Database {
        let path = Path::new(&self.file_path);
        if path.exists() {
            let mut file = OpenOptions::new()
                .read(true)
                .open(path)
                .expect("Failed to open file");
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).expect("Failed to read file");
            Database::decode(&*buffer).expect("Failed to decode database")
        } else {
            Database { tables: Vec::new() }
        }
    }

    // Save the database to file
    pub fn save_database(&self, database: &Database) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.file_path)
            .expect("Failed to open file");

        let mut buffer = Vec::new();
        database
            .encode(&mut buffer)
            .expect("Failed to encode database");

        file.write_all(&buffer).expect("Failed to write to file");
    }

    // Handle the CREATE TABLE command and update the database
    pub fn handle_create_table(&self, command: Command) {
        if let Command::CreateTable { name, columns } = command {
            // Convert columns to protobuf ColumnDefinition
            let column_defs: Vec<ColumnDefinition> =
                columns.into_iter().map(ColumnDefinition::from).collect();
            let table_def = TableDefinition {
                name,
                columns: column_defs,
            };

            // Load the current database, add the table, and save it back
            let mut database = self.load_database();
            database.tables.push(table_def);
            self.save_database(&database);
        }
    }

    // Additional methods for handling other commands can go here
}

