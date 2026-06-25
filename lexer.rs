use std::env;
use std::fs;

#[derive(Debug)]

enum Token {
    LBrace,
    RBrace,
    LParenthesis,
    RParenthesis,
    Semicolon,
    Int,
    Return,
    Identifier(String),
    IntegerLiteral(i64),
}

fn is_integer(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tokens: Vec<Token> = Vec::new();

    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut iter = content.chars().peekable();

    while iter.peek().is_some() {
        let mut word = String::new();

        while let Some(&ch) = iter.peek() {
            if !ch.is_whitespace() && (ch.is_alphanumeric() || ch == '_') {
                word.push(ch);
                iter.next();
            } else {
                if !word.is_empty() {
                    let token = match word.as_str() {
                        "int" => Token::Int,
                        "return" => Token::Return,
                        w if is_integer(w) => Token::IntegerLiteral(w.parse().unwrap()),
                        _ => Token::Identifier(word),
                    };
                    tokens.push(token);
                }

                match ch {
                    '{' => tokens.push(Token::LBrace),
                    '}' => tokens.push(Token::RBrace),
                    '(' => tokens.push(Token::LParenthesis),
                    ')' => tokens.push(Token::RParenthesis),
                    ';' => tokens.push(Token::Semicolon),
                    _ => {}
                }

                break;
            }
        }

        iter.next();
    }

    println!("{:#?}", tokens);
}
