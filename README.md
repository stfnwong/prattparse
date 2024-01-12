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



