



const INPUT: &str = include_str!("./input.txt");
pub fn run() {
    let tokens = tokenize(INPUT);
    println!("{:?}",  tokens);
    let result = parse(tokens);
    println!("{:?}", result);
}

macro_rules! expect_pat {
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
    let mut enabled = true;

    while let Some(_) = iter.peek() {  
        let t = iter.next().unwrap();

        match t {
            Token::Mul if enabled => (),
            Token::Do if matches!(iter.peek(), Some(Token::Not)) => {
                expect_pat!(Token::Not in iter);
                expect_pat!(Token::ParenL in iter);
                expect_pat!(Token::ParenR in iter);
                enabled = false;
                continue;
            },
            Token::Do => {
                expect_pat!(Token::ParenL in iter);
                expect_pat!(Token::ParenR in iter);
                enabled = true;
                continue;
            }
            _ => continue
        }
        

        expect_pat!(Token::ParenL     in iter);
        expect_pat!(&Token::Num(n1)   in iter);
        expect_pat!(Token::Comma      in iter);
        expect_pat!(&Token::Num(n2)   in iter);
        expect_pat!(Token::ParenR     in iter);

        result += n1 * n2;
    }


    result
}




fn tokenize(input: &str) -> Vec<Token> {
    let chars: Vec<_> = input.chars().collect();
    let mut index = 0;
    let mut tokens = vec![];

    while let Some(_) = chars.get(index) {
        let mut token = None;

        loop {
            token = parse_token(&chars, &mut index);

            if token.is_some() {
                break;
            }

            if index == input.len() {
                return tokens;
            }

            tokens.push(Token::Invalid);
            index += 1;
        };

        tokens.push(token.unwrap());
    }


    tokens
}

fn parse_token(chars: &[char], i: &mut usize) -> Option<Token> {
    None
        .or_else(|| tokenize_num(chars, i))
        .or_else(|| tokenize_comma(chars, i))
        .or_else(|| tokenize_paren_l(chars, i))
        .or_else(|| tokenize_paren_r(chars, i))
        .or_else(|| tokenize_not(chars, i))
        .or_else(|| tokenize_do(chars, i))
        .or_else(|| tokenize_mul(chars, i))
}

fn tokenize_do(chars: &[char], i: &mut usize) -> Option<Token> {
    let pattern = "do";
    if tokenize_pattern(pattern, chars, i) {
        Some(Token::Do)
    } else {
        None
    }
}

fn tokenize_not(chars: &[char], i: &mut usize) -> Option<Token> {
    let pattern = "n't";
    if tokenize_pattern(pattern, chars, i) {
        Some(Token::Not)
    } else {
        None
    }
}

fn tokenize_mul(chars: &[char], i: &mut usize) -> Option<Token> {
    let pattern = "mul";
    if tokenize_pattern(pattern, chars, i) {
        Some(Token::Mul)
    } else {
        return None
    }
}



fn tokenize_comma(chars: &[char], i: &mut usize) -> Option<Token> {
    let pattern = ",";
    if tokenize_pattern(pattern, chars, i) {
        Some(Token::Comma)
    } else {
        None
    }
}

fn tokenize_paren_r(chars: &[char], i: &mut usize) -> Option<Token> {
    let pattern = ")";
    if tokenize_pattern(pattern, chars, i) {
        Some(Token::ParenR)
    } else {
        None
    }
}

fn tokenize_paren_l(chars: &[char], i: &mut usize) -> Option<Token> {
    let pattern = "(";
    if tokenize_pattern(pattern, chars, i) {
        Some(Token::ParenL)
    } else {
        None
    }
}


fn tokenize_num(chars: &[char], i: &mut usize) -> Option<Token> {
    let mut string = String::new();
    while let Some('0'..='9') = chars.get(*i) {
        string.push(chars[*i]);
        *i += 1;

        if string.len() > 3 {
            break
        }
    }

    if string.is_empty() {
        return None
    }
    
    Some(Token::Num(string.parse::<i32>().unwrap()))
}

fn tokenize_pattern(pattern: &str, chars: &[char], i: &mut usize) -> bool {
    for p in pattern.chars() {
        if chars.get(*i).is_none_or(|c| *c != p) {
            return false
        }

        *i += 1;
    }

    true
}


#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Do,
    Not,
    Mul,
    Num(i32),
    ParenL,
    ParenR,
    Comma,
    Invalid,
}