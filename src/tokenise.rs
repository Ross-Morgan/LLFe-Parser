use indexmap::IndexMap;

use crate::sections::parse_attr;
use tokens::{Function, Token, Attr};

pub trait Tokenise {
    fn tokenise(&self) -> IndexMap<Function, Vec<Token>>;
}

pub trait TokeniseHeader {
    fn tokenise(&self) -> Function;
}

pub trait TokeniseContents {
    fn tokenise(&self) -> Vec<Token>;
}

impl Tokenise for IndexMap<String, Vec<String>> {
    fn tokenise(&self) -> IndexMap<Function, Vec<Token>> {
        let mut tokenised = IndexMap::new();

        for (section_name, section_contents) in self {
            let tokenised_name = section_name.tokenise();
            let tokenised_contents = section_contents.tokenise();

            tokenised.insert(tokenised_name, tokenised_contents);
        }

        tokenised
    }
}

impl TokeniseHeader for String {
    fn tokenise(&self) -> Function {
        let split = self.split("\n").collect::<Vec<_>>();

        if split.len() == 0 { panic!("No header?"); }

        let mut name;
        let mut attrs = vec![];

        name = split.last().unwrap().to_string();

        if split.len() > 1 {
            attrs = split[0..split.len()-2]
                .into_iter()
                .map(|&s| parse_attr(s))
                .collect();
        }

        Function { name, attrs }
    }
}


impl TokeniseContents for Vec<String> {
    fn tokenise(&self) -> Vec<Token> {
        println!("{self:?}");

        vec![]
    }
}
