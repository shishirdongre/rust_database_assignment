use crate::generated_types::generated_types::{
    cell_value, CellValue, ColumnDefinition, ColumnType as ProtoColumnType, Database, Row,
    TableData, TableDefinition,
};
use crate::nom_parser::{Column, ColumnType, Command, Value};
use crate::table_definition::Table;
use core::panic;
use nom::Err;
use prost::Message;
use std::cell::Cell;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use std::{cell, fmt};

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

    pub fn drop_table(&self, table_name: &str) -> Result<(), DatabaseError> {
        let mut database = self.load_database();
        let table_index = database
            .tables
            .iter()
            .position(|table| table.name == table_name);
        match table_index {
            Some(index) => {
                database.tables.remove(index);
                self.save_database(&database);
                Ok(())
            }
            None => Err(DatabaseError::TableDoesNotExist(table_name.to_string())),
        }
    }

    pub fn display_schema(&self, table_name: &str) -> Result<(), DatabaseError> {
        let database = self.load_database();
        let table = database
            .tables
            .iter()
            .find(|table| table.name == table_name);
        match table {
            Some(table) => {
                println!("Schema for table '{}':", table_name);
                for column in &table.columns {
                    // println!(
                    //     "  - Column: {} | Type: {:?} | Length: {} | Not Null: {}",
                    //     column.name,
                    //     ProtoColumnType::try_from(column.col_type).unwrap(),
                    //     column.length,
                    //     column.not_null
                    // );
                    println!("{:?}", column);
                }
                Ok(())
            }
            None => Err(DatabaseError::TableDoesNotExist(table_name.to_string())),
        }
    }
    pub fn load_table(&self, table_name: &str) -> Result<TableData, DatabaseError> {
        if !self.has_table(table_name) {
            return Err(DatabaseError::TableDoesNotExist(table_name.to_string()));
        }
        let database: Database = self.load_database();
        let file_path = format!("{}.tab", table_name);
        let path = Path::new(&file_path);
        if path.exists() {
            let mut file = OpenOptions::new()
                .read(true)
                .open(format!("{}.tab", table_name))
                .expect("Failed to open file");

            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).expect("Failed to read file");
            let table_data = TableData::decode(&*buffer).expect("Failed to decode database");
            // println!("table_data {:?}", table_data);
            Ok(table_data)
        } else {
            Ok(TableData {
                table_name: table_name.to_string(),
                rows: Vec::new(),
                num_rows: 0,
            })
        }
    }

    pub fn select(&self, command: Command) -> Result<(), DatabaseError> {
        if let Command::Select {
            columns,
            table,
            join_table,
        } = command
        {
            let database = self.load_database();
            if !self.has_table(&table) {
                return Err(DatabaseError::TableDoesNotExist(table));
            }

            let table_definition = database
                .tables
                .iter()
                .find(|table_def| table_def.name == table)
                .expect("Table not found");

            let table_data = self.load_table(&table)?;

            // println!("Table: {:?}", table_data);

            // Print column headers with fixed width of 30 for each column
            for col in table_definition.columns.iter() {
                print!("{:<30}", col.name); // Left-align each column header with padding
            }
            println!();

            // Print separator line with width of 80
            println!("{}", "-".repeat(80));

            // Print each row's values
            for row in table_data.rows {
                for cell in &row.cells {
                    match cell {
                        CellValue {
                            value: Some(cell_value::Value::IntVal(v)),
                        } => print!("{:<30}", v),
                        CellValue {
                            value: Some(cell_value::Value::StrVal(s)),
                        } => print!("{:<30}", s),
                        CellValue {
                            value: Some(cell_value::Value::NullVal(is_null)),
                        } => {
                            if *is_null {
                                print!("{:<30}", "NULL");
                            }
                        }
                        CellValue { value: None } => print!("{:<30}", "Matched None"),
                    }
                }
                println!(); // End of row
            }
            Ok(())
        } else {
            panic!("Invalid command passed to select");
        }
    }

    pub fn insert(&self, command: Command) -> Result<(), DatabaseError> {
        println!("Insert command: {:?}", command);
        if let Command::Insert { table, values } = command {
            let table_data = self.load_table(&table)?;
            let mut new_rows = Vec::new();
            // Convert each `nom_parser::Value` to `CellValue`
            let mut cells = Vec::new();

            for row in values {
                for cell_value in row {
                    if cfg!(debug_assertions) {
                        println!("{:?}", cell_value);
                    }

                    match cell_value {
                        Value::Int(v) => {
                            // println!("int value {:?}", v);
                            cells.push(CellValue {
                                value: Some(cell_value::Value::IntVal(v)),
                            });
                        }
                        Value::Str(s) => {
                            // println!("string value {:?}", s);
                            cells.push(CellValue {
                                value: Some(cell_value::Value::StrVal(s)),
                            });
                        }
                        Value::Null => {
                            // println!("Null value found");
                            cells.push(CellValue {
                                value: Some(cell_value::Value::NullVal(true)),
                            });
                        }
                    }
                    new_rows.push(Row {
                        cells: cells.clone(),
                    });
                }
                // rows.push(table_row);
                // table_row.clear();
                cells.clear();
            }

            let mut all_rows = table_data.rows.clone();
            all_rows.extend(new_rows);
            let len = all_rows.len();

            let table_data = TableData {
                table_name: table,
                rows: all_rows,
                num_rows: len as u32,
            };

            // println!("table_data : {:?}", table_data);

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(format!("{}.tab", table_data.table_name))
                .expect("Failed to open file");

            let mut buffer = Vec::new();
            table_data
                .encode(&mut buffer)
                .expect("Failed to encode database");

            file.write_all(&buffer).expect("Failed to write to file");
            file.sync_all().expect("Failed to sync file");
            Ok(())
        } else {
            panic!("Invalid command passed to insert");
        }
    }
}
