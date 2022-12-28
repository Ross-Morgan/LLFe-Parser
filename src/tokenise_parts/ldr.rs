use errors::LLFeError;
use tokens::Token;

pub fn tokenise(command: Vec<&str>, tokens: &mut Vec<Token>) -> Result<(), LLFeError> {
    let registers = base_tokenise(&command);

    match registers {
        Some(t) => tokens.push(t),
        None => (),
    }

    Ok(())
}


pub fn tokenise_risc(command: Vec<&str>, tokens: &mut Vec<Token>) -> Result<(), LLFeError> {
    let registers = base_tokenise(&command);

    match registers {
        Some(t) => tokens.push(Token::RISC(Box::new(t))),
        None => (),
    }

    Ok(())
}


fn base_tokenise(command: &Vec<&str>) -> Option<Token> {
    let target_reg = command.get(0).expect("No target register specified").trim();
    let source_reg = command.get(1).expect("No source register specified").trim();

    if target_reg == source_reg { return None; }

    if !target_reg.starts_with("r") { panic!("Register missing 'r' identifier"); }
    if !source_reg.starts_with("r") { panic!("Register missing 'r' identifier"); }

    let parsed_target_reg = target_reg.get(1..target_reg.len()).unwrap().parse::<i32>().expect("Invalid target register");
    let parsed_source_reg = source_reg.get(1..source_reg.len()).unwrap().parse::<i32>().expect("Invalid source register");

    Some(Token::LDR(parsed_target_reg, parsed_source_reg))
}

pub fn mode(risc: bool) -> impl Fn(Vec<&str>, &mut Vec<Token>) -> Result<(), LLFeError>
{
    match risc {
        true => tokenise_risc,
        false => tokenise,
    }
}
