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

use lexer::{return_tokens, Token};

#[derive(Debug)]
enum Exp {
    Constant(i64),
}

#[derive(Debug)]
enum Statement {
    Return(Exp),
}

#[derive(Debug)]
enum Function {
    Function { name: String, body: Statement },
}

#[derive(Debug)]
enum Program {
    Program(Function),
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
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

        return Function::Function {
            name: name,
            body: statement,
        };
    }

    fn parse_program(&mut self) -> Program {
        let function = self.parse_function();

        let program = Program::Program(function);

        return program;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut tokens = return_tokens(content);

    println!(
        "{:#?}",
        Parser::parse_program(&mut Parser {
            tokens: tokens,
            pos: 0
        })
    );
}
