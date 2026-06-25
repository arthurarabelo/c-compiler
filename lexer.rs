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

    while let Some(&_c) = iter.peek() {
        let mut word = String::new();

        while let Some(&ch) = iter.peek() {
            if !ch.is_whitespace() && (ch.is_alphanumeric() || ch == '_') {
                word.push(ch);
                iter.next();
            } else {
                if !word.is_empty() {
                    if word == "int" {
                        tokens.push(Token::Int);
                    } else if word == "return" {
                        tokens.push(Token::Return);
                    } else if is_integer(&word) {
                        tokens.push(Token::IntegerLiteral(word.parse::<i64>().unwrap()));
                    } else {
                        tokens.push(Token::Identifier(word));
                    }
                }

                if ch == '{' {
                    tokens.push(Token::LBrace);
                }

                if ch == '}' {
                    tokens.push(Token::RBrace);
                }

                if ch == '(' {
                    tokens.push(Token::LParenthesis);
                }

                if ch == ')' {
                    tokens.push(Token::RParenthesis);
                }

                if ch == ';' {
                    tokens.push(Token::Semicolon);
                }

                break;
            }
        }

        iter.next();
    }

    println!("{:#?}", tokens);
}
