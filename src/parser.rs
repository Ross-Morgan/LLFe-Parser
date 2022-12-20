
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
    pub fn parse(&self) {
        let sections = self.source.sections();
        let tokenized_sections = sections.tokenise();
    }
}
