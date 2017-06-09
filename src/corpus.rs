use token::Token;
use language::{Language, DefaultLanguage};

use std::io;
use std::collections::HashSet;
use std::hash;
use itertools::Itertools;
use std::mem;
use std::convert::TryFrom;

/// A unigram is a single token.
pub type Unigram<'t, L> = Token<'t, L>;

/// A bigram is a tuple of tokens of the same language (and usually
/// the same corpora).
pub type Bigram<'t, L>  = (Token<'t, L>, Token<'t, L>);

/// A line is a slice of tokens.
pub type Line<'t, L> = &'t[Token<'t, L>];

/// The `Document` type, parameterized by a Language.
pub struct Document<L=DefaultLanguage>
  where L: 'static
{
  #[allow(dead_code)]
  bytes: Vec<u8>,
  tokens: Vec<Token<'static, L>>,
  lines: Vec<Line<'static, L>>,
}

impl<L> Document<L> {
  /// Returns a slice of tokens in the document.
  pub fn tokens<'t>(&'t self) -> &'t [Token<'t, L>] {
    &self.tokens[..]
  }

  /// Returns a slice of lines in the document.
  pub fn lines<'t>(&'t self) -> &'t [&'t [Token<'t, L>]] {
    &self.lines[..]
  }
}


impl<I: io::Read, L> TryFrom<I> for Document<L> {
  type Error = io::Error;
  /// Creates a document from a value implementing the [`Read`] trait by
  /// reading bytes until all bytes have been read. For example:
  ///
  /// ```rust
  /// let mut files = env::args().skip(1)
  ///     .map(File::open)
  ///     .map(Result::unwrap)
  ///     .take(2);
  /// 
  /// let original:    Document<French>  = files.next().unwrap().try_into()?;
  /// let translation: Document<English> = files.next().unwrap().try_into()?;
  /// ```
  /// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
  fn try_from(mut i: I) -> Result<Document<L>, io::Error> {
    let mut bytes = vec![];
    i.read_to_end(&mut bytes)?;
    Ok(bytes.into())
  }
}


impl<I: Into<Vec<u8>>, L> From<I> for Document<L> {
  /// Creates a document from any value which can be interpreted as a
  /// vector of bytes.
  ///
  /// ```rust
  ///
  /// let english: Document<English> = "The soup pleased the dog.".into();
  /// let fthishr: Document<Fthishr> = "Zhiidh or thir o vozir.".into();
  /// ```
  /// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
  fn from(i: I) -> Document<L> {
    // Unsafe is used in this function to extend the lifetimes of tokens
    // derived from the `Document` byte vector to that of the lifetime of
    // the entire program. This is necessary because `Document`
    // self-borrows; the `tokens` and `lines` fields contain pointers
    // into the `bytes` field.
    //
    // Accordingly, these fields are kept private and the `tokens()` and
    // `lines()` methods provide a safe interface that constrains
    // the lifetimes of returned values to that of the document they
    // belong to.
    //
    // It is important that after creating the `Document` value, that its
    // `bytes` and `tokens` vectors are never pushed to. Exceeding the
    // internal capacity of either of these vectors would force a
    // reallocation of their backing memory store and thereby invalidate
    // pointers into those vectors. Accordingly, no interface is
    // provided for extending a `Document` with additional tokens after it
    // is initialized.

    let bytes = i.into();

    let mut tokens = vec![];
    let mut lines = vec![];

    for sentence in bytes.split(|&c| c == b'\n') {
      let s = tokens.len();
      tokens.extend(sentence.split(|&c| c == b' ')
        .filter(|w| !w.is_empty())
        .map(|w| unsafe {mem::transmute::<Token<L>,_>(w.into())}));
      let e = tokens.len();
      lines.push((s,e));
    }

    let lines = lines.iter().map(|&(s,e)|
      unsafe{mem::transmute(&tokens[s..e])}).collect_vec();

    Document {
      bytes: bytes,
      tokens: tokens,
      lines: lines,
    }
  }
}

/// Consumes an iterator over tokens and a vocabulary, and produces
/// an iterator over tokens in which all unknown tokens (tokens that are 
/// not in the given vocabulary) are replaced with [`Token::Unknown`].
/// [`Token::Unknown`]: enum.Token.html#variant.Unknown
pub fn unk<'t, T, L, S>(tokens: T, vocabulary: &'t  HashSet<Token<'t, L>, S>)
    -> impl 't + Iterator<Item=Token<'t, L>>
  where L: Language + 't,
        T: 't + IntoIterator<Item=Token<'t, L>>,
        S: hash::BuildHasher
{
  IntoIterator::into_iter(tokens)
    .map(move |word| 
      if vocabulary.contains(&word) { word } 
      else { Token::Unknown })
}

/// Consumes an iterator over tokens and produces the same iterator over
/// tokens.
pub fn unigrams<'t, T, L>(tokens: T)
    -> impl Iterator<Item=Token<'t, L>>
  where L: Language + 't,
        T: IntoIterator<Item=Token<'t, L>>{
  IntoIterator::into_iter(tokens)
}

/// Consumes an iterator over tokens and produces an iterator over all
/// bigrams (adjacent tokens) in the input stream.
pub fn bigrams<'t, T, L>(tokens: T)
    -> impl Iterator<Item=Bigram<'t, L>>
  where L: Language + 't,
        T: IntoIterator<Item=&'t Token<'t, L>> {
  IntoIterator::into_iter(tokens).cloned().tuple_windows::<(_,_)>()
}

/// Consumes an interator over lines, and produces an iterator over
/// all tokens in the document, with [`Token::Null`] values inserted at
/// sentence boundaries.
///
/// # Example
///
/// ```rust
/// extern crate nlptk;
/// extern crate itertools;
/// use nlptk::{Document, padded};
/// use itertools::Itertools;
///
/// fn main() {
///   let testing : Document = "The soup pleased the dog.
///                           The cat caught the rat.".into();
///
///   assert_eq!(padded(testing.lines()).join(" "),
///     "ε The soup pleased the dog. ε The cat caught the rat. ε");
/// }
/// ```
///
/// [`Token::Null`]: enum.Token.html#variant.Null
pub fn padded<'t, I, L: 't + Language>(lines: I)
    -> impl 't + Iterator<Item=Token<'t, L>>
  where I: 't + IntoIterator<Item=&'t &'t[Token<'t, L>]>
{
  use std::iter::once;
  once(Token::Null).chain(
    IntoIterator::into_iter(lines)
      .map(|sentence| sentence.iter().cloned().chain(once(Token::Null)))
      .flatten())
}

impl<'t, L: 't> IntoIterator for &'t Document<L> {
  type Item = &'t &'t [Token<'t, L>];
  type IntoIter = ::std::slice::Iter<'t, &'t [Token<'t, L>]>;

  /// Convert a reference to a document into an iterator over lines
  /// in the document.
  fn into_iter(self) -> Self::IntoIter {
    IntoIterator::into_iter(self.lines())
  }
}

impl<'t, L: 't + Language> Into<&'t [&'t [Token<'t, L>]]> for &'t Document<L> {
  /// Convert a reference to a document into a reference to a slice of all
  /// lines in the document.
  fn into(self) -> &'t [&'t [Token<'t, L>]] {
    self.lines()
  }
}

impl<'t, L: 't + Language> Into<&'t [Token<'t, L>]> for &'t Document<L> {
  /// Convert a reference to a document into a reference to a slice of all
  /// tokens in the document.
  fn into(self) -> &'t [Token<'t, L>] {
    self.tokens()
  }
}
