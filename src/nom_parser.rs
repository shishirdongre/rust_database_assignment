use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
enum Command {
    CreateTable {
        name: String,
        columns: Vec<Column>,
    },
    DeleteTable {
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
}

#[derive(Debug)]
enum ColumnType {
    INT,
    STRING(u32),
}

#[derive(Debug)]
struct Column {
    name: String,
    col_type: ColumnType,
    not_null: bool,
}

// Utility parsers
fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

// Updated column_type parser to include optional length for STRING
fn column_type(input: &str) -> IResult<&str, ColumnType> {
    alt((
        map(tag("INT"), |_| ColumnType::INT),
        map(delimited(tag("STRING("), digit1, tag(")")), |len: &str| {
            ColumnType::STRING(len.parse::<u32>().unwrap_or(256))
        }),
        map(tag("STRING"), |_| ColumnType::STRING(256)), // Default STRING length is 256 if not specified
    ))(input)
}

// Parser for "CREATE TABLE" command with optional "NOT NULL" for columns
fn create_table(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("CREATE TABLE")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, columns) = separated_list0(
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

    Ok((
        input,
        Command::CreateTable {
            name: name.to_string(),
            columns,
        },
    ))
}

fn delete_table(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("DELETE FROM")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = identifier(input)?;
    Ok((
        input,
        Command::DeleteTable {
            name: name.to_string(),
        },
    ))
}

fn list_table(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("LIST TABLE")(input)?;
    Ok((input, Command::ListTable))
}

fn list_schema(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("LIST SCHEMA")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = identifier(input)?;
    Ok((
        input,
        Command::ListSchema {
            name: name.to_string(),
        },
    ))
}

fn select_columns(input: &str) -> IResult<&str, Vec<String>> {
    alt((
        map(tag("*"), |_| vec!["*".to_string()]),
        separated_list0(tag(", "), map(identifier, String::from)),
    ))(input)
}

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
    Ok((
        input,
        Command::Select {
            columns,
            table: table.to_string(),
            join_table: join_table.map(|s| s.to_string()),
        },
    ))
}

// Top-level parser for any command
fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((
        create_table,
        delete_table,
        list_table,
        list_schema,
        select_statement,
    ))(input)
}

// Test case to parse and pretty-print each command
pub fn run_parser() {
    let tests = vec![
        "CREATE TABLE users (id INT, name STRING(20))",
        "DELETE FROM orders",
        "LIST TABLE",
        "LIST SCHEMA users",
        "SELECT * FROM users JOIN orders",
        "SELECT id, name FROM users",
        "CREATE TABLE users (id INT, name STRING(20) NOT NULL, age INT NOT NULL)",
        "CREATE TABLE products (code STRING(10), price INT)",
    ];

    for test in tests {
        match parse_command(test) {
            Ok((_, command)) => println!("{:?}", command),
            Err(e) => println!("Error parsing '{}': {:?}", test, e),
        }
    }
}
