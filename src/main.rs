use parser::{parser::Parser, program::Program};

const SOURCE: &'static str = "

#[thumb]
function_1:
    Actually useful code

#[attr]
function_2:
    mov r7, #0x1
    mov r0, #0x65

function_3:
    run exit
";


fn main() {
    let mut program = Program::new();

    let parser = Parser {
        source: SOURCE.to_string(),
        program: &mut program
    };

    parser.parse();
}
