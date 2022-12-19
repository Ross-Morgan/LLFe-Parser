use parser::{parser::Parser, program::Program};

const SOURCE: &'static str = "
function_1:
    Actually useful code

function_2:
    mov r7, #0x1
    mov r0, #0x65

function_3:
";


fn main() {
    let mut consts = vec![];
    let mut vars = vec![];
    let mut functions = vec![];

    let mut program = Program {
        consts: &mut consts,
        vars: &mut vars,
        functions: &mut functions,
    };

    let parser = Parser {
        source: SOURCE.to_string(),
        program: &mut program
    };

    parser.parse()
}
