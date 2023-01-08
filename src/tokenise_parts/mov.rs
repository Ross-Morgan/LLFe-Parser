use tokens::Token;

use super::literal::{self, parse_register, parse_immediate};
use errors::{LLFeError, new_error};


pub fn tokenise(command: Vec<&str>, tokens: &mut Vec<Token>) -> Result<(), LLFeError> {
    let data = base_tokenise(&command)?;

    match data {
        Some(token) => (),
        None => (),
    }

    Ok(())
}

pub fn tokenise_risc(command: Vec<&str>, tokens: &mut Vec<Token>) -> Result<(), LLFeError> {
    let data = base_tokenise(&command);

    if data.is_err() {
        return Err(new_error(format!("Failed to tokenise command (risc): {command:?}"), Some(Box::new(data.unwrap_err()))));
    }

    match data.unwrap() {
        Some(token) => {
            tokens.push(token);
            Ok(())
        },
        None => Ok(()),
    }
}

fn base_tokenise(command: &Vec<&str>) -> Result<Option<Token>, LLFeError> {
    let target = command.get(0).expect("No target register specified").trim().trim_end_matches(",");
    let source = command.get(1).expect("No source specified").trim();

    if target.get(0..1) != Some("r") { return Err(new_error(format!("Missing register specifier: {target:?}"), None)); }

    let parsed_target = match parse_target(target) {
        Ok(t) => t,
        Err(e) => return Err(new_error(format!("Failed to parse target: {target:?}"), Some(Box::new(e)))),
    };

    let parsed_source = match parse_source(source) {
        Ok(t) => t,
        Err(e) => return Err(new_error(format!("Failed to parsed source: {source:?}"), Some(Box::new(e)))),
    };

    Ok(Some(Token::MOV(Box::new(parsed_target), Box::new(parsed_source))))
}


fn parse_target(source: &str) -> Result<Token, LLFeError> {
    if source.is_empty() {
        return Err(new_error(format!("No target specified in {source:?}"), None));
    }

    let r = match source.get(0..1).unwrap() {
        "#" => parse_immediate(source),
        "r" => parse_register(source),
        _ => Ok(Token::Register(0)),
    };

    match r {
        Ok(t) => Ok(t),
        Err(e) => Err(new_error(format!("Failed to parse line: {source:?}"), Some(Box::new(e))))
    }
}


fn parse_source(source: &str) -> Result<Token, LLFeError> {
    if source.len() < 2 {
        match source {
            "#" => return Err(new_error(format!("No immediate specified: {source:?}"), None)),
            "r" => return Err(new_error(format!("No register specified: {source:?}"), None)),
            _ => return Err(new_error(format!("Invalid identifier: {source:?}"), None)),
        }
    }

    if source.starts_with("#") {
        literal::parse_immediate(source)
    } else if source.starts_with("=") {
        Ok(Token::VarRef(String::from("<placeholder var>")))
    } else {
        Err(new_error("No immediate or reference specified", None))
    }
}


pub fn mode(risc: bool) -> impl Fn(Vec<&str>, &mut Vec<Token>) -> Result<(), LLFeError>
{
    match risc {
        true => tokenise_risc,
        false => tokenise,
    }
}
