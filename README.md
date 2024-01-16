# Pratt Parsing
I found [this](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html) article on Pratt Parsers and wanted to follow along.

This is just a shitty cargo project that basically copies the blog post. I reproduce some of the notes here for my own posterity. 
If this is in a public repo and you are looking at it then you might as well just look at the original blog post since I am a hack.


### Recursive Descent Parsing

The idea in recursive descent parsing is that we model the grammar as a set of mutually
recursive functins.  For instance, if we have a grammar like 

```
Expr ::= 
        Factor 
    |   Expr '+' Expr 


Factor ::= 
        Atom 
    |   Factor '*' Atom


Atom ::= 
        'number'
    |   '(' Expr ')'
```


Then we can model that fragment with something like 

```rust
fn item(p: &mut Parser)
{
    match p.peek() {
        STRUCT_KEYWORD => struct_item(p),
        ENUM_KEYWORD => enum_item(p)
        // etc...
    }
}


fn struct_item(p: &mut Parser)
{
    p.expect(STRUCT_KEYWORD);
    name(p);
    p.expect(LEFT_BRACE);
    field_list(p);
    p.expect(RIGHT_BRACE);
}

// etc...
```

This fails for left-recursive grammars. For instance, if we have the grammar

```
Sum ::= 
        Sum '+' Int
    |   Int
```

And we implement

```rust
fn sum(p: &mut Parser) {
    // Try first alternative 
    sum(p);         // <- DUN GOOFED! We loop again immediately and overflow the stack
    p.expect(PLUS);
    int(p);
}
```

For a hand-written parser, one solution is to use an iterative implementation

```rust
fn sum(p: &mut Parser) {
    sum(p);

    while p.eat(PLUS) {
        int(p);
    }
}
```



### "Binding power" model

Imagine the expression `A + B * C`. We expect that the `*` binds tighter than the `+`,
and so we expect the expression to be parsed as `A + (B * C)`. 

Rather than try to think about this expression in terms of precedence, imagine that 
each operator has a numerical "binding power". The larger this number the more "powerful"
binding is for that operator, and more powerful operators are bound ahead of less powerful
ones.

An example using the expression `A + B * C`:

```
expr:   A   +   B   *   C
power:    3   3   5   5
```

For operators that have the same binding power we can model which one to fold by 
making the powers slightly asymmetric (basically, by making the power on the right
slightly higher).

```
expr:     A   +     B   +      C
power: 0    3   3.1   3   3.1     0   # <- Note the (implicit) leading and trailng zero
```

Now the first `+` holds its operands tighter than its neighbours, and so we can fold
the first expression into `(A + B)`.

```
expr:     (A+B)   +      C
power: 0        3   3.1     0
```

The second `+` has a slightly higher binding power on the right and so prefers to bind
to `C`. The first `+` captures both `A` and `B`.
