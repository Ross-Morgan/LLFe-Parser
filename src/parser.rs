
use tokens::{Const, Data, Var, FunctionBuilder, Token};

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
        let tokenized_sections = sections.tokenise();

        let mut const_buffer = vec![];
        let mut var_buffer = vec![];

        tokenized_sections
            .unwrap()
            .into_iter()
            .for_each(|(mut fb, c)| {
                fb.contents = Some(c);

                match fb.name.as_ref().unwrap().as_str() {
                    "consts" => parse_data_section::<Const>(&mut const_buffer),
                    "vars" => parse_data_section::<Var>(&mut var_buffer),
                    _ => self.program.functions.push(fb.build()),
                };
            });
    }
}


fn parse_data_section<'a, T: Data>(contents: &Vec<Token>, v: &'a mut Vec<T>) -> &'a mut Vec<T> {
    contents
        .iter()
        .map(|t|)

    v
}
