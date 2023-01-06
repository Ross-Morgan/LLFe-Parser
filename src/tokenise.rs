use indexmap::IndexMap;

use errors::{LLFeError, new_error};
use tokens::{Token, Attr, FunctionBuilder};

use crate::sections::parse_attr;
use crate::tokenise_parts;


pub trait Tokenise {
    fn tokenise(&self) -> Result<IndexMap<FunctionBuilder, Vec<Token>>, LLFeError>;
}

pub trait TokeniseHeader {
    fn tokenise(&self) -> Result<FunctionBuilder, LLFeError>;
}

pub trait TokeniseContents {
    fn tokenise(&self, risc: bool) -> Result<Vec<Token>, LLFeError>;
}


impl Tokenise for IndexMap<String, Vec<String>> {
    fn tokenise(&self) -> Result<IndexMap<FunctionBuilder, Vec<Token>>, LLFeError> {
        let mut tokenised = IndexMap::new();

        for (section_name, section_contents) in self {
            let builder = match section_name.tokenise() {
                Err(e) => return Err(new_error("", Some(Box::new(e)))),
                Ok(fb) => fb,
            };

            let risc = builder.attrs.is_some() && builder.attrs.clone().unwrap().contains(&Attr { name: "thumb".to_string(), value: None });

            let contents = match section_contents.tokenise(risc) {
                Err(e) => return Err(new_error("", Some(Box::new(e)))),
                Ok(t) => t,
            };

            tokenised.insert(builder, contents);
        }

        Ok(tokenised)
    }
}


impl TokeniseHeader for String {
    fn tokenise(&self) -> Result<FunctionBuilder, LLFeError> {
        let mut split = self.split("\n").collect::<Vec<_>>();

        if split.len() == 0 { panic!("No header?"); }

        let name = split.last().unwrap().to_string();
        let mut attrs = vec![];

        if split.len() > 1 {
            split.remove(split.len() - 1);
            attrs = split
                .into_iter()
                .map(parse_attr)
                .collect();
        }

        Ok(FunctionBuilder { name: Some(name), attrs: Some(attrs), contents: None })
    }
}


impl TokeniseContents for Vec<String> {
    fn tokenise(&self, risc: bool) -> Result<Vec<Token>, LLFeError> {
        let mut tokens = vec![];

        for line in self.clone().into_iter() {
            let r = tokenise_line(line, risc, &mut tokens);

            match r {
                Ok(_) => (),
                Err(e) => return Err(new_error("Failed to parse line", Some(Box::new(e))))
            }
        }

        Ok(tokens)
    }
}


pub fn tokenise_line(line: String, risc: bool, tokens: &mut Vec<Token>) -> Result<(), LLFeError> {
    let mut command = line.split(" ").collect::<Vec<_>>();

    if command.is_empty() { return Ok(()); }

    let first = command.remove(0);

    match first {
        "mov" => {
            let r = tokenise_parts::mov::mode(risc)(command, tokens);

            match r {
                Ok(_) => (),
                Err(e) => return Err(new_error(format!("Failed to parse `mov` command: {line:?}"), Some(Box::new(e)))),
            };

            tokens.push(Token::NOP);
            // TODO Add result to token vector
        },
        "ldr" => {
            let r = tokenise_parts::ldr::mode(risc)(command, tokens);

            match r {
                Ok(_) => (),
                Err(e) => return Err(new_error(format!("Failed to parse `ldr` command: {line:?}"), Some(Box::new(e))))
            }

            tokens.push(Token::NOP);
            // TODO Add result to token vector
        },
        "run" => {
            let mut v = command
                .into_iter()
                .map(|s| Token::FuncRef(s.into()))
                .collect::<Vec<_>>();

            tokens.append(&mut v);

        },
        c => {
            println!("Invalid command {c}");
            return Err(new_error("Invalid command", None))
        },
    };

    Ok(())
}
