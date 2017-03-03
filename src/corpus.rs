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

/// The `Corpus` type, parameterized by a Language.
pub struct Corpus<L=DefaultLanguage>
  where L: 'static
{
  #[allow(dead_code)]
  bytes: Vec<u8>,
  words: Vec<Token<'static, L>>,
  sentences: Vec<&'static [Token<'static, L>]>,
}

/// A sentence is a slice of words.
pub type Sentence<'t, L> = &'t[Token<'t, L>];

impl<L> Corpus<L> {
  /// Returns a slice of tokens in the document.
  pub fn words<'t>(&'t self) -> &'t [Token<'t, L>] {
    &self.words[..]
  }

  /// Returns a slice of sentences in the document.
  pub fn sentences<'t>(&'t self) -> &'t [&'t [Token<'t, L>]] {
    &self.sentences[..]
  }
}


impl<I: io::Read, L> TryFrom<I> for Corpus<L> {
  type Err = io::Error;
  /// Creates a corpus from a value implementing the [`Read`] trait by
  /// reading bytes until all bytes have been read. For example:
  ///
  /// ```rust
  /// let mut files = env::args().skip(1)
  ///     .map(File::open)
  ///     .map(Result::unwrap)
  ///     .take(2);
  /// 
  /// let original:    Corpus<French>  = files.next().unwrap().try_into()?;
  /// let translation: Corpus<English> = files.next().unwrap().try_into()?;
  /// ```
  /// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
  fn try_from(mut i: I) -> Result<Corpus<L>, io::Error> {
    let mut bytes = vec![];
    i.read_to_end(&mut bytes)?;
    Ok(bytes.into())
  }
}


impl<I: Into<Vec<u8>>, L> From<I> for Corpus<L> {
  /// Creates a corpus from any value which can be interpreted as a
  /// vector of bytes.
  ///
  /// ```rust
  ///
  /// let english: Corpus<English> = "The soup pleased the dog.".into();
  /// let fthishr: Corpus<Fthishr> = "Zhiidh or thir o vozir.".into();
  /// ```
  /// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
  fn from(i: I) -> Corpus<L> {
    // Unsafe is used in this function to extend the lifetimes of tokens
    // derived from the `Corpus` byte vector to that of the lifetime of
    // the entire program. This is necessary because `Corpus`
    // self-borrows; the `words` and `sentences` fields contain pointers
    // into the `bytes` field.
    //
    // Accordingly, these fields are kept private and the `words()` and
    // `sentences()` methods provide a safe interface that constrains
    // the lifetimes of returned values to that of the corpus they
    // belong to.
    //
    // It is important that after creating the `Corpus` value, that its
    // `bytes` and `words` vectors are never pushed to. Exceeding the
    // internal capacity of either of these vectors would force a
    // reallocation of their backing memory store and thereby invalidate
    // pointers into those vectors. Accordingly, no interface is
    // provided for extending a `Corpus` with additional tokens after it
    // is initialized.

    let bytes = i.into();

    let mut words = vec![];
    let mut sentences = vec![];

    for sentence in bytes.split(|&c| c == b'\n') {
      let s = words.len();
      words.extend(sentence.split(|&c| c == b' ')
        .filter(|w| !w.is_empty())
        .map(|w| unsafe {mem::transmute::<Token<L>,_>(w.into())}));
      let e = words.len();
      sentences.push((s,e));
    }

    let sentences = sentences.iter().map(|&(s,e)|
      unsafe{mem::transmute(&words[s..e])}).collect_vec();

    Corpus {
      bytes: bytes,
      words: words,
      sentences: sentences,
    }
  }
}

/// Consumes an iterator over tokens and a vocabulary, and produces
/// an iterator over tokens in which all unknown words (words that are 
/// not in the given vocabulary) are replaced with [`Token::Unknown`].
/// [`Token::Unknown`]: enum.Token.html#variant.Unknown
pub fn unk<'t, T, L, S>(words: T, vocabulary: &'t  HashSet<Token<'t, L>, S>)
    -> impl 't + Iterator<Item=Token<'t, L>>
  where L: Language + 't,
        T: 't + IntoIterator<Item=Token<'t, L>>,
        S: hash::BuildHasher
{
  IntoIterator::into_iter(words)
    .map(move |word| 
      if vocabulary.contains(&word) { word } 
      else { Token::Unknown })
}

/// Consumes an iterator over tokens and produces the same iterator over
/// tokens.
pub fn unigrams<'t, T, L>(words: T)
    -> impl Iterator<Item=Token<'t, L>>
  where L: Language + 't,
        T: IntoIterator<Item=Token<'t, L>>{
  IntoIterator::into_iter(words)
}

/// Consumes an iterator over tokens and produces an iterator over all
/// bigrams (adjacent tokens) in the input stream.
pub fn bigrams<'t, T, L>(words: T)
    -> impl Iterator<Item=Bigram<'t, L>>
  where L: Language + 't,
        T: IntoIterator<Item=Token<'t, L>> {
  IntoIterator::into_iter(words).tuple_windows::<(_,_)>()
}

/// Consumes an interator over sentences, and produces an iterator over
/// all words in the corpus, with [`Token::Null`] values inserted at
/// sentence boundaries.
///
/// # Example
///
/// ```rust
/// extern crate nlptk;
/// extern crate itertools;
/// use nlptk::{Corpus, padded};
/// use itertools::Itertools;
///
/// fn main() {
///   let testing : Corpus = "The soup pleased the dog.
///                           The cat caught the rat.".into();
///
///   assert_eq!(padded(testing.sentences()).join(" "),
///     "ε The soup pleased the dog. ε The cat caught the rat. ε");
/// }
/// ```
///
/// [`Token::Null`]: enum.Token.html#variant.Null
pub fn padded<'t, I, L: 't + Language>(sentences: I)
    -> impl 't + Iterator<Item=Token<'t, L>>
  where I: 't + IntoIterator<Item=&'t &'t[Token<'t, L>]>
{
  use std::iter::once;
  once(Token::Null).chain(
    IntoIterator::into_iter(sentences)
      .map(|sentence| sentence.iter().cloned().chain(once(Token::Null)))
      .flatten())
}
