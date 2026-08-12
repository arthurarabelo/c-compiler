use std::env;
use std::fs;
use std::io;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::path::Path;

mod parser;
use parser::{ Parser, Program, Function, Statement, Exp, FunctionInfo };

fn generate_assembly (program: Program) -> io::Result<()> {
    let function = match program {
        Program::Program(Function::Function( FunctionInfo { name, body } )) => {
            FunctionInfo {name: name, body: body}
        }
    };

    let return_value = match function.body {
        Statement::Return(Exp::Constant(n)) => n
    };

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("assembly.s")?;

    let mut writer = std::io::BufWriter::new(file);

    writeln!(writer, ".globl {}", function.name)?;
    writeln!(writer, "{}:", function.name)?;
    writeln!(writer, "movl ${}, %eax", return_value)?;
    writeln!(writer, "ret")?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = Path::new(&args[1]);
    let file_name = file_path.with_extension("");
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let tokens = parser::lexer::return_tokens(content);
    let program = Parser::parse_program(&mut Parser {
        tokens: tokens,
        pos: 0
    });

    let _ = generate_assembly(program);

    let _ = Command::new("gcc")
        .arg("-m32")
        .arg("assembly.s")
        .arg("-o")
        .arg(file_name)
        .output()
        .expect("Failed to compile the program");

    let _ = Command::new("rm")
        .arg("assembly.s")
        .output()
        .expect("Failed to remove the program");
}
