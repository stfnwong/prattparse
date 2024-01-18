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


// Tokenization section
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


// Note that the return contains an empty left binding power to make
// clear that this is a prefix operator (and not a postfix operator)
// and can therefore only bind to the right.
fn prefix_binding_power(op: char) -> ((), u8)
{
    match op {
        '+' | '-' => ((), 9),
        _ => panic!("bad op {:?}", op),
    }
}

fn postfix_binding_power(op: char) -> Option<(u8, ())>
{
    let res = match op {
        '!' => (11, ()),
        '[' => (11, ()),
        _ => return None,
    };

    Some(res)
}

fn infix_binding_power(op: char) -> Option<(u8, u8)>
{
    let res = match op {
        '=' => (2, 1),      // C-style assignment operator
        '?' => (4, 3),
        '+' | '-' => (5, 6),
        '*' | '/' => (7, 8),
        // High-priority right-associative function composition operato
        // Note that adding this one line is enough to implement the correct 
        // precendence behaviour for this operator. Because the operator binds
        // "higher" we automatically get the correct right-associativity.
        '.' => (14, 13),          // NOTE: we need to bring these up higher than '!'
        _ => return None,
    };

    Some(res)
}

// Start with two infix binary operators 
fn expr(input: &str) -> S 
{
    let mut lexer = Lexer::new(input);
    // We use a binding power of zero here to start recursion
    expr_bp(&mut lexer, 0)     
}

/*
 * expr_bp()
 *
 */
fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S 
{
    let mut lhs = match lexer.next() {
        Token::Atom(it) => S::Atom(it),
        // Add paren expressions here. These are just primary expressions that are 
        // handled like ops.
        Token::Op('(') => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        },
        Token::Op(op) => {
            let ((), right_bp) = prefix_binding_power(op);  
            let rhs = expr_bp(lexer, right_bp);  // right_bp used for rexcursive calls
            S::Cons(op, vec![rhs])
        },
        t => panic!("bad token {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token {:?}", t),
        };

        // First try to return a postfix operator 
        if let Some((left_bp, ())) = postfix_binding_power(op) {
            if left_bp < min_bp {
                break;
            }
            lexer.next();

            // We can add an array indexing operator here by observing that in a 
            // expression like a[i], the 'i' doesn't really 'bind' to anything from 
            // the parsers point of view. In a sense, the '[]' part of the expression
            // is like a paren expression in the location of a postfix operator.
            lhs = if op == '[' {
                let rhs = expr_bp(lexer, 0);
                assert_eq!(lexer.next(), Token::Op(']'));       // TODO: what to really use here
                                                                // instead of assetrt_eq! ?
                S::Cons(op, vec![lhs, rhs])
            } else {
                S::Cons(op, vec![lhs])
            };

            continue;
        }

        // Using the parameter min_bp here allows us to break the loop
        // once we see an operator weaker than the current one. We then 
        // return back up the stack to the weak point and call lexer.next()
        // to make the next recursive call.
        // In a way, min_bp represents the binding power of the operator
        // to the left of the current expression.

        // NOTE: Return None here on unrecognised operands. We do this 
        // to create a terminating condition similar to postfix_binding_power.
        if let Some((left_bp, right_bp)) = infix_binding_power(op) {
            if left_bp < min_bp {
                break;
            }

            lexer.next();

            // Here we check the infix '?' operator
            lhs = if op == '?' {
                let mhs = expr_bp(lexer, 0);
                assert_eq!(lexer.next(), Token::Op(':'));
                let rhs = expr_bp(lexer, right_bp);
                S::Cons(op, vec![lhs, mhs, rhs])
            } else {
                let rhs = expr_bp(lexer, right_bp);
                S::Cons(op, vec![lhs, rhs])
            };

            continue;
        }
        break;
    }

    return lhs;
}




#[test]
fn tests() {
    let s = expr("1");
    assert_eq!(s.to_string(), "1");

    let s = expr("1 + 2 * 3");
    assert_eq!(s.to_string(), "(+ 1 (* 2 3))");

    let s = expr("a + b * c * d + e");
    assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");

    // Test the function composition operator '.'
    let s = expr("f . g .h ");
    assert_eq!(s.to_string(), "(. f (. g h))");

    // This works even with the other operators 
    let s = expr("1 + 2 + f . g . h * 3 * 4");
    assert_eq!(s.to_string(), "(+ (+ 1 2) (* (* (. f (. g h)) 3) 4))");

    let s = expr("--1 * 2");
    assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

    let s = expr("--f . g");
    assert_eq!(s.to_string(), "(- (- (. f g)))");

    let s = expr("-9!");
    assert_eq!(s.to_string(), "(- (! 9))");

    let s = expr("f . g !");
    assert_eq!(s.to_string(), "(! (. f g))");

    // Parenthesis expression test 
    let s = expr("(((0)))");
    assert_eq!(s.to_string(), "0");

    let s = expr("a[i][j]");
    assert_eq!(s.to_string(), "([ ([ a i) j)");

    let s = expr("a ? b : c ? d : e");
    assert_eq!(s.to_string(), "(? a b (? c d e))");  // not of course there are 3 operands

    let s = expr("a = 0 ? b : c = d");
    assert_eq!(s.to_string(), "(= a (= (? 0 b c) d))");
}


// === TESTS ==== //
// TODO: move to another module?

fn main() {
    let s = expr("1 + 2 * 3");
    println!("{:?}", s.to_string());
}
