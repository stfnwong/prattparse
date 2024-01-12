// General shape of a Pratt parser is to use both loops and recursion
//
// fn parse_expr() {
//      ...
//      loop {
//          ...
//          parse_expr()
//          ...
//      }
//  }

use std::fmt;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Atom(char),
    Op(char),
    Eof,
}


struct Lexer {
    tokens: Vec<Token>
}


impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' |
                'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Op(c),
            })
            .collect::<Vec<_>>();
        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        return self.tokens.pop().unwrap_or(Token::Eof);
    }

    fn peek(&mut self) -> Token {
        return self.tokens.last().copied().unwrap_or(Token::Eof);
    }
}


// Transform to S-Expression
enum S {
    Atom(char),
    Cons(char, Vec<S>),
}


impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}


fn infix_binding_power(op: char) -> (u8, u8)
{
    match op {
        '+' | '-' => (1, 2),
        '*' | '/' => (3, 4),
        _ => panic!("bad op {:?}", op)
    }
}

// Start with two infix binary operators 
fn expr(input: &str) -> S 
{
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer, 0)
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S 
{
    let mut lhs = match lexer.next() {
        Token::Atom(it) => S::Atom(it),
        t => panic!("bad token {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token {:?}", t),
        };

        let (left_bp, right_bp) = infix_binding_power(op);
        if left_bp < min_bp {
            break;
        }

        lexer.next();
        let rhs = expr_bp(lexer, right_bp);

        lhs = S::Cons(op, vec![lhs, rhs]);
    }

    return lhs;
}




#[test]
fn tests() {
    let s = expr("1");
    assert_eq!(s.to_string(), "1");

    let s = expr("1 + 2 * 3");
    assert_eq!(s.to_string(), "(+ 1 (* 2 3))");
}

// === TESTS ==== //
// TODO: move to another module?

fn main() {
    let s = expr("1 + 2 * 3");
    println!("{:?}", s.to_string());
}
