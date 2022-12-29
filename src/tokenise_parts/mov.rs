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
    let data = base_tokenise(&command)?;

    match data {
        Some(token) => (),
        None => (),
    }

    Ok(())
}

fn base_tokenise(command: &Vec<&str>) -> Result<Option<Token>, LLFeError> {
    let target = command.get(0).expect("No target register specified").trim();
    let source = command.get(1).expect("No source specified").trim();

    if target.get(0..1) != Some("r") { return Err(new_error("Missing register specified", None)); }

    let parsed_target = match parse_target(target) {
        Ok(t) => t,
        Err(e) => return Err(new_error("", Some(Box::new(e)))),
    };

    let parsed_source = match parse_source(source) {
        Ok(t) => t,
        Err(e) => return Err(new_error("", Some(Box::new(e)))),
    };

    Ok(Some(Token::MOV(Box::new(parsed_target), Box::new(parsed_source))))
}


fn parse_target(source: &str) -> Result<Token, LLFeError> {
    if source.is_empty() {
        return Err(new_error("No target specified", None));
    }

    let r = match source.get(0..1).unwrap() {
        "#" => parse_immediate(source),
        "r" => parse_register(source),
        _ => Ok(Token::REGISTER(0)),
    };


    Ok(Token::NOP)
}


fn parse_source(source: &str) -> Result<Token, LLFeError> {
    if source.len() < 2 {
        match source {
            "#" => return Err(new_error("No immediate specified", None)),
            "r" => return Err(new_error("No register specified", None)),
            _ => return Err(new_error("Invalid identifier", None)),
        }
    }

    if source.len() < 4 {
        if source.chars().all(|c| c.is_numeric()) {
            return Ok(Token::INT32(source.parse().unwrap()));
        } else {
            return Err(new_error("", None));
        }
    }

    if source.starts_with("#") {
        match literal::parse_immediate(source) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        }
    } else if source.starts_with("=") {
        // Parse variable
        Ok(Token::VAR_REF(String::from("")))
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