use crate::sections::SplitSections;
use crate::program::Program;

type MutRefProgram<'a> = &'a mut Program<'a>;

pub struct Parser<'a> {
    pub source: String,
    pub program: MutRefProgram<'a>
}

impl<'a> Parser<'a> {
    pub fn new(source: String, program: MutRefProgram<'a>) -> Self {
        Self { source, program }
    }

    pub fn parse_from_source(source: String, program: MutRefProgram<'a>) {
        let p = Self { source, program };

        p.parse()
    }
}

impl<'a> Parser<'a> {
    pub fn parse(&self) {
        let sections = self.sections();

        println!("{sections:#?}")
    }
}
