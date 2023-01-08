use errors::{LLFeError, new_error};
use tokens::Token;

pub fn parse_immediate(immediate: &str) -> Result<Token, LLFeError> {
    if !immediate.starts_with("#") {
        return Err(new_error(format!("Immediate missing '#' identifier: {immediate:?}"), None));
    }

    if immediate.len() < 2 {
        return Err(new_error(format!("Immediate missing value {immediate:?}"), None));
    }

    match immediate.get(1..3) {
        Some(v) => {
            let parsed = match v {
                "0x" if immediate.len() > 3 => parse_base(immediate.get(3..immediate.len()).unwrap(), 16),
                "0o" if immediate.len() > 3 => parse_base(immediate.get(3..immediate.len()).unwrap(), 08),
                "0b" if immediate.len() > 3 => parse_base(immediate.get(3..immediate.len()).unwrap(), 02),
                _ => return Err(new_error(format!("Invalid base specifier: {immediate:?}"), None)),
            };

            match parsed {
                Ok(n) => Ok(Token::Int32(n)),
                Err(e) => return Err(new_error(format!("Failed to parse value: {v:?}"), Some(Box::new(e))))
            }
        }
        None => {
            match immediate.get(1..2).unwrap().parse::<i32>() {
                Ok(i) => Ok(Token::Int32(i)),
                Err(_) => Err(new_error(format!("Invalid immediate specified: {immediate:?}"), None)),
            }
        }
    }
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
                Ok(i) => Ok(Token::Register(i)),
                Err(_) => Err(new_error(format!("Failed to parse register: {register:?}"), None)),
            }
        }
    }
}


fn parse_base(src: &str, base: u32) -> Result<i32, LLFeError> {
    match i32::from_str_radix(src, base) {
        Ok(n) => Ok(n),
        Err(_) => Err(new_error(format!("Failed to parsed base: {src:?} in base {base}"), None))
    }
}
