# nlptk
[_Documentation_](https://jswrenn.gitlab.io/nlptk)   

A natural language processing toolkit designed to aid in the
implementation of Brown University's [_Introduction to Computational
Linguistics_](http://cs.brown.edu/courses/csci1460/). This toolkit
provides only corpora management utilities.

This toolkit requires the rust nightly compiler.

## Using this Library
Add the following to your project's `Cargo.toml` file:

```
[dependencies.nlptk]
git = "https://gitlab.com/jswrenn/nlptk"
version = "0.1.0"
```

## Language Annotations for Corpora and Tokens
This toolkit uses [phantom types][spooky] to tag corpora and tokens with
what language they are in. It is easy to accidentally reverse the order
of arguments of a lookup into a `HashMap` from words of one language to
words of another, for example. This technique enables the type checker
to statically prevents such mistakes. For example, this code will compile:

```rust
fn get_translation_probability<'t, L, M>(
    translation_table : HashMap<(Token<'t, L>, Token<'t, M>), f64>, 
    foreign_word      : Token<'t, L>,
    native_word       : Token<'t, M>)
      -> f64
  where L: Language,
        M: Language
{
  translation_table
    .get(&(foreign_word, native_word))
    .unwrap_or(0.0)
}
```
but this code, which reverses the order of `foreign_word` and 
`native_word` will not:

```rust
fn get_translation_probability<'t, L, M>(
    translation_table : HashMap<(Token<'t, L>, Token<'t, M>), f64>, 
    foreign_word      : Token<'t, L>,
    native_word       : Token<'t, M>)
      -> f64
  where L: Language,
        M: Language
{
  translation_table
    .get(&(foreign_word, native_word))
    .unwrap_or(0.0)
}
```

[spooky]: http://rustbyexample.com/generics/phantom.html

## Running the Example
This repository contains an [example generative unigram model][unigram].
Given a training corpus, this program generates sentences of a 
probabilistically appropriate length, containing the probabilistically 
appropriate words. To run this example, clone this repository and
execute this in the root project directory:

```bash
cargo run --example unigram -- examples/data/penn-tree-bank-train.txt | more
```

[unigram]: https://gitlab.com/jswrenn/nlptk/blob/master/examples/unigram.rs




