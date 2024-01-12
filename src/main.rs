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


fn main() {
    println!("Hello, world!");
}
