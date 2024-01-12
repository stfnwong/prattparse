/// An example recursive desent parser.
///
/// The idea is to model the grammar as a set of mutually recursive 
/// recursive functions.


// For example, imagine the grammar fragment 
// Expr ::= 
//      Expr '+' Expr 
//  |   Expr '*' Expr
//  |   '(' Expr ')'
//  |   'number'
//
//  This isn't precise enough for automatic parser generation. We need to re-write
//  to specifiy the precedence and associativity. For example:
//
// Expr ::= 
//      Factor
//  |   Expr '+' Factor
//
//  Factor ::=
//      Atom 
//  |   Factor '*' Atom
//
//  Atom ::= 
//      'number'
//  |   '(' Expr ')'


// We can model this fragment like;

fn item(p: &mut: Parser) 
{
    match p.peek() {
        STRUCT_KEYWORD => struct_item(p),
        ENUM_KEYWORD => struct_item(p),
    }
}


fn struct_item(p: &mut Parser)
{
    p.expect(STRUCT_KEYWORD);
    name(p);
    p.expect(L_BRACE);
    field_list(p);
    p.expect(R_BRACE);
}
