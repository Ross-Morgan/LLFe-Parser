use errors::{LLFeError, new_error};
use tokens::Token;

pub fn parse_immediate(immediate: &str) -> Result<Token, LLFeError> {
    todo!();
}

pub fn parse_register(register: &str) -> Result<Token, LLFeError> {
    if register.get(0..1) != Some("r") {
        return Err(new_error("Register missing 'r' identifier", None));
    }

    match register.get(1..register.len()) {
        None => return Err(new_error("No register specified", None)),
        Some(s) if s.is_empty() => return Err(new_error("No register specified", None)),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(i) => Ok(Token::REGISTER(i)),
                Err(_) => Err(new_error("Register could not be parsed", None)),
            }
        }
    }
}
