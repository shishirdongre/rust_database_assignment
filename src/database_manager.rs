use crate::generated_types::generated_types::{
    ColumnDefinition, ColumnType as ProtoColumnType, Database, TableDefinition,
};
use crate::nom_parser::{Column, ColumnType, Command};
use core::panic;
use nom::Err;
use prost::Message;
use std::error::Error;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

// Define a custom error type
#[derive(Debug)]
pub enum DatabaseError {
    TableAlreadyExists(String),
    TableDoesNotExist(String),
    NoTablesInDatabase,
    IOError(std::io::Error),
    UnknownError,
}

// Implement Display for DatabaseError
impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::TableAlreadyExists(name) => {
                write!(f, "Table '{}' already exists.", name)
            }
            DatabaseError::IOError(err) => write!(f, "I/O error: {}", err),
            DatabaseError::TableDoesNotExist(name) => {
                write!(f, "Table '{}' does not exist.", name)
            }
            DatabaseError::NoTablesInDatabase => write!(f, "No tables in the database."),
            DatabaseError::UnknownError => write!(f, "An unknown error occurred."),
        }
    }
}

// Implement the Error trait for DatabaseError
impl Error for DatabaseError {}

// Convert std::io::Error into DatabaseError::IOError for convenience
impl From<std::io::Error> for DatabaseError {
    fn from(error: std::io::Error) -> Self {
        DatabaseError::IOError(error)
    }
}

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

    pub fn has_table(&self, table_name: &str) -> bool {
        let database = self.load_database();
        database.tables.iter().any(|table| table.name == table_name)
    }

    // Handle the CREATE TABLE command and update the database
    pub fn create_table(&self, command: Command) -> Result<(), DatabaseError> {
        if let Command::CreateTable { name, columns } = command {
            // Convert columns to protobuf ColumnDefinition
            if self.has_table(&name) {
                return Err(DatabaseError::TableAlreadyExists(name));
            }
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
            Ok(())
        } else {
            panic!("Invalid command passed to create_table");
        }
    }

    // Implement `list_table` to display all tables in the database
    pub fn list_tables(&self) -> Result<(), DatabaseError> {
        // Load the current database from the file
        let database = self.load_database();

        // Check if there are any tables in the database
        if database.tables.is_empty() {
            Err(DatabaseError::NoTablesInDatabase)
        } else {
            println!("Tables in the database:");
            for table in &database.tables {
                println!("{}", table.name);
                // println!("- Table: {}", table.name);
                // println!("  Columns:");
                // for column in &table.columns {
                //     println!(
                //         "    - Column: {} | Type: {:?} | Length: {} | Not Null: {}",
                //         column.name,
                //         ProtoColumnType::try_from(column.col_type).unwrap(),
                //         column.length,
                //         column.not_null
                //     );
                // }
                // println!();
            }
            Ok(())
        }
    }
}
