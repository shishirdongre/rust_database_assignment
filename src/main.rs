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

use clap::Parser as ClapParser;
use column_definition::{Column, ColumnType};
use database_manager::{DatabaseError, DatabaseManager};
use file_manager::FileManager;
use nom_parser::{parse_command, Command as ParsedCommand};
use parser::Parser;
use table_definition::Table;
use token::{Token, TokenType};

// Define the CLI structure with `clap`
#[derive(ClapParser, Debug)]
#[clap(about = "A simple database manager CLI")]
struct Cli {
    /// SQL command to execute (e.g., "CREATE TABLE ...")
    #[clap()]
    command: String,
}

fn main() {
    // Step 1: Parse the command-line arguments
    let args = Cli::parse();

    // Step 2: Initialize the DatabaseManager
    let db_manager = DatabaseManager::new("dbfile.bin".to_string());

    // Step 3: Parse the SQL command
    let parsed_command = match parse_command(&args.command) {
        Ok((_, command)) => command,
        Err(e) => {
            eprintln!("Error parsing command: {:?}", e);
            return;
        }
    };

    // Step 4: Match parsed command to call DatabaseManager methods
    match parsed_command {
        ParsedCommand::CreateTable { name, columns } => {
            match db_manager.create_table(ParsedCommand::CreateTable {
                // Cloning here to be able to print it below
                name: name.clone(),
                columns,
            }) {
                Ok(_) => println!("Table created successfully."),
                Err(e) => eprintln!("{}", e),
            }
        }
        ParsedCommand::DropTable { name } => match db_manager.drop_table(&name) {
            Ok(_) => println!("Table '{}' dropped .", name),
            Err(e) => eprintln!("{}", e),
        },
        ParsedCommand::ListTable => match db_manager.list_tables() {
            Ok(_) => println!("Tables listed successfully."),
            Err(e) => eprintln!("{}", e),
        },
        ParsedCommand::ListSchema { name } => {
            println!("List schema for table '{}' - Not implemented", name);
            match db_manager.display_schema(&name) {
                Ok(_) => {}
                Err(e) => eprintln!("{}", e),
            }
        }
        ParsedCommand::Select {
            columns,
            table,
            join_table,
        } => {
            println!("Select query on table '{}' - Not implemented", table);
        }
    }

    // Step 5: Print the updated database for verification
    let database = db_manager.load_database();
    // println!("Current Database: {:#?}", database.tables);
}
