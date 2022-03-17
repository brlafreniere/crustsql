#[derive(PartialEq, Eq)]
enum PrimaryCommand {
  SELECT,
  CREATE,
  UPDATE,
  DELETE,
  DROP
}

pub struct Query<'a> {
  primary_command: PrimaryCommand,
  primary_arguments: Vec<&'a str>,
  from_clause: &'a str,
  where_claus: &'a str
}

impl<'a> Query<'a> { }

pub struct Parser { }

impl Parser {
  fn get_primary_command(query_str: &str) -> Option<PrimaryCommand> {
    let first_word: &str = &(query_str.split(" ").nth(0).unwrap().to_lowercase());
    match first_word {
      "select" => Some(PrimaryCommand::SELECT),
      _ => None
    }
  }

  pub fn parse(input_str: &str) -> Option<Query> {
    let primary_command = Parser::get_primary_command(input_str);
    match primary_command {
      Some(primary_command) => {
        let query = Query {
          primary_command: primary_command,
          primary_arguments: Vec::new(),
          from_clause: "",
          where_claus: ""
        };
        return Some(query);
      }
      _ => None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parser_returns_query_object() {
    let input_string = "SELECT * FROM table;";
    let result = Parser::parse(input_string);
    assert!(result.unwrap().primary_command == PrimaryCommand::SELECT);
    println!("{}", input_string);
  }
}