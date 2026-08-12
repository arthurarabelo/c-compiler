/* A ideia é aninhar os pub enums de cada non-terminal:

Program::Program(
    Function::Function {
        name: "main".to_string(),
        body: Statement::Return(Exp::Constant(2)),
    },
)

*/

// TODO: instead of using match and fail, maybe I could use the operator ? and Result<T, E>

use std::env;
use std::fs;

#[path = "lexer.rs"] pub mod lexer;
use self::lexer::{return_tokens, Token};

#[derive(Debug)]
pub enum Exp {
    Constant(i64),
}

#[derive(Debug)]
pub enum Statement {
    Return(Exp),
}

#[derive(Debug)]
pub struct FunctionInfo {
    pub name: String,
    pub body: Statement
}

#[derive(Debug)]
pub enum Function {
    Function(FunctionInfo)
}

#[derive(Debug)]
pub enum Program {
    Program(Function),
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

impl Parser {
    fn fail(&mut self, msg: &str) -> ! {
        eprintln!("Parse error: {msg}");
        std::process::exit(1);
    }

    fn advance(&mut self) -> Option<&Token> {
        let old_pos = self.pos;
        self.pos += 1;

        return self.tokens.get(old_pos);
    }

    fn expect(&mut self, expected: Token, msg: &str) {
        match self.advance() {
            Some(token) if *token == expected => {}
            _ => self.fail(msg),
        };
    }

    fn parse_exp(&mut self) -> Exp {
        let exp = match self.advance() {
            Some(Token::IntegerLiteral(n)) => Exp::Constant(*n),
            _ => self.fail("error parsing the expression: constant expected"),
        };

        return exp;
    }

    fn parse_statement(&mut self) -> Statement {
        self.expect(Token::Return, "error parsing statement: return expected");

        let exp = self.parse_exp();

        self.expect(Token::Semicolon, "error parsing statement: semicolon expected");

        return Statement::Return(exp);
    }

    fn parse_function(&mut self) -> Function {
        self.expect(Token::Int, "error parsing function: int expected");

        let name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => self.fail("error parsing function: identifier expected"),
        };

        self.expect(Token::LParenthesis, "error parsing function: ( expected");

        self.expect(Token::RParenthesis, "error parsing function: ) expected");

        self.expect(Token::LBrace, "error parsing function: { expected");

        let statement = self.parse_statement();

        self.expect(Token::RBrace, "error parsing function: } expected");

        return Function::Function(
            FunctionInfo {
                name: name,
                body: statement
            }
        );
    }

    pub fn parse_program(&mut self) -> Program {
        let function = self.parse_function();

        let program = Program::Program(function);

        return program;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let tokens = return_tokens(content);

    println!(
        "{:#?}",
        Parser::parse_program(&mut Parser {
            tokens: tokens,
            pos: 0
        })
    );
}
