mod column_definition;
mod file_manager;
mod nom_parser;
mod parser;
mod semantic_processor;
mod table_definition;
mod table_manager;
mod token;

use column_definition::{Column, ColumnType};
use file_manager::FileManager;
use parser::Parser;
use table_definition::Table;
use token::{Token, TokenType};

use nom_parser::run_parser;

fn main() {
    // let file_manager = FileManager::new("dbfile.bin");
    // match file_manager.read_file() {
    //     Ok(data) => {
    //         // Process data as necessary
    //         println!("Read data from file: {:?}", data);
    //     }
    //     Err(e) => {
    //         println!("Error reading file: {}", e);
    //     }
    // }

    // // Example of creating a parser and parsing a SQL command
    // let mut parser = Parser::new("CREATE TABLE example (id INT NOT NULL, name VARCHAR(20))");
    // let tokens = parser.parse_tokens();
    // for token in tokens {
    //     println!("{:?}", token);
    // }

    // // Example of creating a column and table definition
    // let id_column = Column::new("id", ColumnType::Int, true);
    // let name_column = Column::new("name", ColumnType::Varchar(20), false);
    // let mut table = Table::new("example", vec![id_column, name_column]);

    // println!("Table Definition: {:?}", table);
    run_parser();
}
