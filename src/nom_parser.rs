use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
pub enum Value {
    Int(i64),
    Str(String),
    Null,
}

#[derive(Debug)]
pub enum Command {
    CreateTable {
        name: String,
        columns: Vec<Column>,
    },
    DropTable {
        name: String,
    },
    ListTable,
    ListSchema {
        name: String,
    },
    Select {
        columns: Vec<String>,
        table: String,
        join_table: Option<String>,
    },
    Insert {
        table: String,
        values: Vec<Vec<Value>>,
    },
}

#[derive(Debug)]
pub enum ColumnType {
    INT,
    STRING(u64),
}

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub col_type: ColumnType,
    pub not_null: bool,
}

// Utility parsers
fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

fn integer(input: &str) -> IResult<&str, Value> {
    map(digit1, |s: &str| Value::Int(s.parse().unwrap()))(input)
}

fn string(input: &str) -> IResult<&str, Value> {
    map(
        delimited(char('\''), take_while1(|c| c != '\''), char('\'')),
        |s: &str| Value::Str(s.to_string()),
    )(input)
}

fn null_value(input: &str) -> IResult<&str, Value> {
    map(tag("NULL"), |_| Value::Null)(input)
}

// Parse a single value
fn value(input: &str) -> IResult<&str, Value> {
    alt((integer, string, null_value))(input)
}

// Parser for a single tuple of values
fn value_tuple(input: &str) -> IResult<&str, Vec<Value>> {
    delimited(char('('), separated_list1(tag(", "), value), char(')'))(input)
}

// Column type parser to include optional length for STRING
fn column_type(input: &str) -> IResult<&str, ColumnType> {
    alt((
        map(tag("INT"), |_| ColumnType::INT),
        map(delimited(tag("STRING("), digit1, tag(")")), |len: &str| {
            ColumnType::STRING(len.parse::<u64>().unwrap_or(256))
        }),
        map(tag("STRING"), |_| ColumnType::STRING(256)), // Default STRING length is 256 if not specified
    ))(input)
}

// CREATE TABLE parser with optional "NOT NULL" for columns
fn create_table(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("CREATE TABLE")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, columns) = separated_list1(
        tag(", "),
        map(
            tuple((
                map(identifier, String::from),
                map(preceded(multispace0, column_type), |col_type| col_type),
                opt(preceded(multispace0, tag("NOT NULL"))),
            )),
            |(name, col_type, not_null)| Column {
                name,
                col_type,
                not_null: not_null.is_some(),
            },
        ),
    )(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = opt(char(';'))(input)?; // Optional semicolon

    Ok((
        input,
        Command::CreateTable {
            name: name.to_string(),
            columns,
        },
    ))
}

// DROP TABLE parser
fn drop_table(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("DROP TABLE")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = opt(char(';'))(input)?; // Optional semicolon
    Ok((
        input,
        Command::DropTable {
            name: name.to_string(),
        },
    ))
}

// LIST TABLE parser
fn list_table(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("LIST TABLES")(input)?;
    let (input, _) = opt(char(';'))(input)?; // Optional semicolon
    Ok((input, Command::ListTable))
}

// SCHEMA parser
fn display_schema(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("SCHEMA")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = opt(char(';'))(input)?; // Optional semicolon
    Ok((
        input,
        Command::ListSchema {
            name: name.to_string(),
        },
    ))
}

// SELECT statement parser
fn select_statement(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("SELECT")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, columns) = select_columns(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("FROM")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, table) = identifier(input)?;
    let (input, join_table) = opt(preceded(
        pair(multispace0, tag("JOIN")),
        preceded(multispace0, identifier),
    ))(input)?;
    let (input, _) = opt(char(';'))(input)?; // Optional semicolon
    Ok((
        input,
        Command::Select {
            columns,
            table: table.to_string(),
            join_table: join_table.map(|s| s.to_string()),
        },
    ))
}

// Helper parser for SELECT columns
fn select_columns(input: &str) -> IResult<&str, Vec<String>> {
    alt((
        map(tag("*"), |_| vec!["*".to_string()]),
        separated_list0(tag(", "), map(identifier, String::from)),
    ))(input)
}

// Parser for INSERT INTO command with support for multiple tuples
fn insert_into(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("INSERT INTO")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, table) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("VALUES")(input)?;
    let (input, _) = multispace0(input)?;

    // Parse one or more tuples, separated by commas
    let (input, values) = separated_list1(tag(", "), value_tuple)(input)?;
    let (input, _) = opt(char(';'))(input)?; // Optional semicolon

    Ok((
        input,
        Command::Insert {
            table: table.to_string(),
            values,
        },
    ))
}

// Top-level parser for any command
pub fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((
        create_table,
        drop_table,
        list_table,
        display_schema,
        select_statement,
    ))(input)
}

// Test cases to parse and pretty-print each command
pub fn run_parser() {
    let tests = vec![
        "CREATE TABLE users (id INT, name STRING(20));",
        "DELETE FROM orders;",
        "LIST TABLES;",
        "SCHEMA users;",
        "SELECT * FROM users JOIN orders;",
        "SELECT id, name FROM users;",
        "CREATE TABLE users (id INT, name STRING(20) NOT NULL, age INT NOT NULL);",
        "CREATE TABLE products (code STRING(10), price INT);",
    ];

    for test in tests {
        match parse_command(test) {
            Ok((_, command)) => println!("{:?}", command),
            Err(e) => println!("Error parsing '{}': {:?}", test, e),
        }
    }
}
