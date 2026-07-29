/* A ideia é aninhar os enums de cada non-terminal:

Program::Program(
    Function::Function {
        name: "main".to_string(),
        body: Statement::Return(Exp::Constant(2)),
    },
)

*/

use std::env;
use std::fs;

mod lexer;

use lexer::{Token, return_tokens};


#[derive(Debug)]
enum Exp {
   Constant(i64)
}

#[derive(Debug)]
enum Statement {
    Return(Exp)
}

#[derive(Debug)]
enum Function {
    Function {
        name: String,
        body: Statement,
    },
}

#[derive(Debug)]
enum Program {
    Program(Function),
}

fn fail(msg: &str) -> ! {
    eprintln!("Parse error: {msg}");
    std::process::exit(1);
}

fn parse_exp(tokens: &mut Vec<Token>) -> Exp {
    let token = tokens.remove(0);

    let exp = match token {
        Token::IntegerLiteral(n) => Exp::Constant(n),
        _ => fail("error parsing the expression: constant expected"),
    };

    return exp;
}

fn parse_statement(tokens: &mut Vec<Token>) -> Statement {
    let mut token = tokens.remove(0);
    if token != Token::Return {
        fail("error parsing statement: return expected");
    }
    
    let exp = parse_exp(tokens);
    let statement = Statement::Return(exp);

    token = tokens.remove(0);
    if token != Token::Semicolon {
        fail("error parsing statement: semicolon expected");
    }
 
    return statement;
}

fn parse_function (tokens: &mut Vec<Token>) -> Function{
    let mut token = tokens.remove(0);
    if token != Token::Int {
        fail("error parsing function: int expected");
    }

    token = tokens.remove(0);
    let name = match token {
        Token::Identifier(n) => n,
        _ => fail("error parsing function: identifier expected"),
    };

    token = tokens.remove(0);
    if token != Token::LParenthesis {
        fail("error parsing function: ( expected");
    }

    token = tokens.remove(0);
    if token != Token::RParenthesis  {
        fail("error parsing function: ) expected");
    }

    token = tokens.remove(0);
    if token != Token::LBrace  {
        fail("error parsing function: { expected");
    }

    let statement = parse_statement(tokens);

    token = tokens.remove(0);
    if token != Token::RBrace  {
        fail("error parsing function: } expected");
    }
    
    let function = Function::Function {
        name: name,
        body: statement
    };

    return function;
}

fn parse_program(tokens: &mut Vec<Token>) -> Program {
    let function = parse_function(tokens);

    let program = Program::Program(function);

    return program;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut tokens = return_tokens(content);

    println!("{:#?}", parse_program(&mut tokens));
}