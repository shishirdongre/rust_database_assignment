mod column_definition;
mod database_manager;
mod file_manager;
mod generated_types;
mod nom_parser;
mod parser;
mod semantic_processor;
mod table_definition;
mod table_manager;
mod token;

use column_definition::{Column, ColumnType};
use database_manager::DatabaseManager;
use file_manager::FileManager;
use parser::Parser;
use table_definition::Table;
use token::{Token, TokenType};

use nom_parser::{parse_command, Command as ParsedCommand};

// fn main() {
//     // let file_manager = FileManager::new("dbfile.bin");
//     // match file_manager.read_file() {
//     //     Ok(data) => {
//     //         // Process data as necessary
//     //         println!("Read data from file: {:?}", data);
//     //     }
//     //     Err(e) => {
//     //         println!("Error reading file: {}", e);
//     //     }
//     // }

//     // // Example of creating a parser and parsing a SQL command
//     // let mut parser = Parser::new("CREATE TABLE example (id INT NOT NULL, name VARCHAR(20))");
//     // let tokens = parser.parse_tokens();
//     // for token in tokens {
//     //     println!("{:?}", token);
//     // }

//     // // Example of creating a column and table definition
//     // let id_column = Column::new("id", ColumnType::Int, true);
//     // let name_column = Column::new("name", ColumnType::Varchar(20), false);
//     // let mut table = Table::new("example", vec![id_column, name_column]);

//     // println!("Table Definition: {:?}", table);
//     run_parser();
// }

// // Usage example
// fn main() {
//     // Initialize the DatabaseManager
//     let db_manager = DatabaseManager::new("dbfile.bin".to_string());

//     // Example CREATE TABLE command
//     let command = Command::CreateTable {
//         name: "users".to_string(),
//         columns: vec![
//             Column {
//                 name: "id".to_string(),
//                 col_type: ColumnType::INT,
//                 not_null: true,
//             },
//             Column {
//                 name: "name".to_string(),
//                 col_type: ColumnType::STRING(20),
//                 not_null: false,
//             },
//         ],
//     };

//     // Process the command to add the table to the database
//     db_manager.handle_create_table(command);

//     // Load and print the database to verify
//     let database = db_manager.load_database();
//     println!("Database: {:?}", database);
// }

fn main() {
    // Step 1: Define the CREATE TABLE command as a string for parsing
    let create_employee_table = "CREATE TABLE employees (id INT NOT NULL, name STRING(50), age INT, department STRING(30), salary INT);";

    // Step 2: Run the parser on the command string
    let parsed_command = match parse_command(create_employee_table) {
        Ok((_, command)) => command,
        Err(e) => {
            eprintln!("Error parsing command: {:?}", e);
            return;
        }
    };

    // Step 3: Print the parsed command to confirm
    println!("Parsed command: {:?}", parsed_command);

    // Step 4: Initialize the DatabaseManager
    let db_manager = DatabaseManager::new("dbfile.bin".to_string());

    // Step 5: Handle the CREATE TABLE command and add the table definition to the database
    if let ParsedCommand::CreateTable { name, columns } = parsed_command {
        db_manager.handle_create_table(ParsedCommand::CreateTable { name, columns });
    } else {
        eprintln!("Parsed command was not a CREATE TABLE command.");
        return;
    }

    // Step 6: Load the database from the file and print it to verify
    let database = db_manager.load_database();
    println!("Database after adding 'employees' table: {:?}", database);

    // Re-initialize the DatabaseManager
    let db_manager = DatabaseManager::new("dbfile.bin".to_string());

    // Load the database and iterate over each table to print its details
    let database = db_manager.load_database();

    println!("Database tables: {:#?}", database.tables);
    println!("Number of tables: {:#?}", database.tables.len());
}
