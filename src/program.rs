use tokens::{Attr, Const, Function, Token, Var};

type MutRefVec<'a, T> = &'a mut Vec<T>;

pub struct Program<'a> {
    pub consts: MutRefVec<'a, Const>,
    pub vars: MutRefVec<'a, Var>,
    pub functions: MutRefVec<'a, Function>
}


impl<'a> Program<'a> {
    pub fn add_constant(&mut self, name: String, value: Token) {
        self.consts.push(Const { name, value })
    }

    pub fn add_variable(&mut self, name: String, value: Token) {
        self.vars.push(Var { name, value })
    }

    pub fn add_function(&mut self, name: String, attrs: Vec<Attr>) {
        self.functions.push(Function { name, attrs })
    }
}
