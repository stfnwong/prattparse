# Pratt Parsing
I found [this](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html) article on Pratt Parsers and wanted to follow along.

This is just a shitty cargo project that basically copies the blog post. I reproduce some of the notes here for my own posterity. 
If this is in a public repo and you are looking at it then you might as well just look at the original blog post since I am a hack.


### Recursive Descent Parsing

The idea in recursive descent parsing is that we model the grammar as a set of mutually
recursive functions.  For instance, if we have a grammar like 

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


### On array operator ([])
So there are ways to handle prefix, infix and postfix operators. The trick to something
like a `[]` operator is rather than thinking of `a[i]` as a prefix `[`, an infix index 
`i` and a postfix `]` to think of it as a kind of `()` operator. The `i` doesn't
really have anything to do with the binding power of the operator itself. In an expression
like `a[i]` its kind of like a `[]` operator that is postfix of `a`, but with an `i` 
inside. Therefore we can treat the `[i]` as following `a`, and then treat the `[]` like 
a parenthesis with a token inside. We can ignore binding/precedence since the `i` is 
unambiguous - its just an expression we consume before we see a `]`.

### On Ternary Operator 
So the "hardest" one to handle is the ternary operator, which is often something like
`cond ? e1 : e2`. Again the trick is to try and re-think this in terms of other operators
that we have implemented previously. One alternative way to look at this is to write

`cond ? e1 : e2`

as 

`cond [e1] e2`

which makes it appear like a `[]` operator. The trick with `[]` was that it was postfix
followed by parenthesis, and so `? :` can be thought of as a pair of parenthesis. 

What should the associativity be? Consider the example

`a ? b : c ? d : e`

Since we can write `cond ? e1 : e2` as `cond [e1] e2`, we can ignore the parenthesied
part of the expression (which we just parse like `[e1]`) and focus on the `cond` chain.
This gives

`a ?: c ?: e`

We can choose parse this as either

`(a ?: c) ?: e`   (left-associative)

or as 

`a ?: (c ?: e)`   (right-associative)

It turns out that the right-associative reading is the more useful one. The priority of
the `?` operator should also be low (this is the case in C for example). Implementing
this operator with right-associativity is more useful for chains of operators like

```
a ? b :
c ? d :
e
```

