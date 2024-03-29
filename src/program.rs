use tokens::prelude::{Attr, Const, Function, NewData, Token, Var};

#[derive(Clone, Debug, Default)]
pub struct Program {
    pub consts: Vec<Const>,
    pub vars: Vec<Var>,
    pub functions: Vec<Function>,
}


impl Program {
    pub fn new() -> Self {
        Self::default()
    }
}


impl Program {
    pub fn add_constant(&mut self, name: String, value: Token) {
        self.consts.push(Const::new(name, value))
    }

    pub fn add_variable(&mut self, name: String, value: Token) {
        self.vars.push(Var::new(name, value))
    }

    pub fn add_function(&mut self, name: String, attrs: Vec<Attr>, contents: Vec<Token>) {
        self.functions.push(Function { name, attrs, contents })
    }
}
