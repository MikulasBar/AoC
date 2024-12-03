use std::{iter::Peekable, str::Chars};



const INPUT: &str = include_str!("./input.txt");

pub fn run() {
    let tokens = tokenize(INPUT);
    let result = parse(tokens);
    println!("{:?}", result);
}

macro_rules! expect_token {
    ($pat:pat in $iter:expr) => {
        #[allow(irrefutable_let_patterns)]
        let Some($pat) = $iter.peek() else {
            continue
        };
        $iter.next()
    };
}

fn parse(tokens: Vec<Token>) -> i32 {
    let mut iter = tokens.into_iter().peekable();
    let mut result = 0;

    while let Some(t) = iter.peek() {
        if *t != Token::Mul {
            iter.next();
            continue
        }
        
        iter.next();
        expect_token!(Token::ParenL     in iter);
        expect_token!(&Token::Num(n1)   in iter);
        expect_token!(Token::Comma      in iter);
        expect_token!(&Token::Num(n2)   in iter);
        expect_token!(Token::ParenR     in iter);

        result += n1 * n2;
    }


    result
}




fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = vec![];

    while let Some(c) = chars.peek() {
        let token = match c {
            '(' => {
                chars.next();
                Token::ParenL
            },

            ')' => {
                chars.next();
                Token::ParenR
            },

            ',' => {
                chars.next();
                Token::Comma
            }

            '0'..='9' => {
                let mut string = String::new();

                tokenize_seq_while(&mut string, &mut chars, |c| matches!(c, '0'..='9'));

                if string.len() > 3 {
                    Token::Invalid
                } else {
                    Token::Num(string.parse::<i32>().unwrap())
                }
            },

            'm' => {
                let mut string = String::new();

                tokenize_seq_while(&mut string, &mut chars, |c| matches!(c, 'a'..='z'));

                if string.as_str() == "mul" {
                    Token::Mul
                } else {
                    Token::Invalid
                }
            },

            _ => {
                chars.next();
                Token::Invalid
            },
        };

        tokens.push(token);
    }


    tokens
}


fn tokenize_seq_while(buffer: &mut String, chars: &mut Peekable<Chars<'_>>, cond: fn(char) -> bool) {
    while let Some(&char) = chars.peek() {
        if cond(char) {
            buffer.push(char);
            chars.next();
        } else {
            break;
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Mul,
    Num(i32),
    ParenL,
    ParenR,
    Comma,
    Invalid,
}