use tokens::DataBuilder;
use tokens::prelude::{Const, Data, NewData, Token, Var};

use crate::program::Program;
use crate::sections::SplitSections;
use crate::tokenise::Tokenise;


pub struct Parser<'a> {
    pub source: String,
    pub program: &'a mut Program
}

impl<'a> Parser<'a> {
    pub fn new(source: String, program: &'a mut Program) -> Self {
        Self { source, program }
    }

    pub fn parse_from_source(source: String, program: &'a mut Program) {
        Self { source, program }.parse();
    }
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) {
        let sections = self.source.sections();

        let tokenized_sections = match sections.tokenise() {
            Ok(i) => i,
            Err(e) => {
                println!("{e}");
                return;
            }
        };

        let mut const_buffer = vec![];
        let mut var_buffer = vec![];

        tokenized_sections
            .into_iter()
            .for_each(|(mut fb, c)| {
                fb.contents = Some(c.clone());

                match fb.name.as_ref().unwrap().as_str() {
                    "consts" => parse_data_section::<Const>(&c, &mut const_buffer),
                    "vars" => parse_data_section::<Var>(&c, &mut var_buffer),
                    _ => self.program.functions.push(fb.build()),
                };
            });
    }
}


fn parse_data_section<T: NewData + Data>(contents: &Vec<Token>, _v: &mut Vec<T>) {
    let builder = DataBuilder::default();

    // TODO add tokens for data

    dbg!(contents);
    todo!();
}
