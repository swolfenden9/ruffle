# Ruffle

A silly little programming language like Rust, Zig, and C# combined. *Sigh* Yeah, I know...

Ruffle is a compiled language, implemented in Rust. Its syntax is **very** similar to rust with a few exeptions:
- Variables and functions don't have `:` or `->` after them to specify types
- `?` and `!` to denote optional values or error unions (like Zig)
- Uh.. dunno

The language also includes some extra features not present in base Rust such as:
- OOP - The language includes inheritance
- Events - Similar to C# `Action`s

## Goals

- [ ] Lexer
- [ ] Recursive descent parser into AST
- [ ] Semantic analysis

## Examles

### Syntax

#### Variable Declarations

```ruffle
let x = 69;
let (x, y) = (2, 3);
let x u8 = 33;
let x;
```

#### Functions

```ruffle
fn main() {
  // some boring old crap
}

fn dox(ip IpAdress, how_bad Badness) { }

fn returns_something() bool { }

fn could_be_none() i32? { }

enum Error { /* --snip-- */ }

fn could_be_error() !Error { }

fn something_or_error() Badness!Error { }

// note: shit starts to get real

fn something_maybe_or_error() i32?!Error { }

fn something_or_error_maybe() i32!Error?

fn something_or_maybe_error() i32!(Error?)

fn something_or_maybe_error_maybe() i32!(Error?)?

// And for the final boss...

// note: you should never, ever make anything like this
fn something_or_maybe_error_or_error_maybe_error_maybe() i32!(Error?)!(Error?!Error)? { }
```
