use std::fmt;
use std::marker::PhantomData;
use language::{Language, DefaultLanguage};

const UNK_CHARS: [u8;5] = [b'*', b'U', b'N', b'K', b'*'];
pub static UNK: Token<'static, DefaultLanguage> 
  = Token::Word(Word{chars: &UNK_CHARS, language: PhantomData});

/// A `Word` is a slice of characters belonging from a `Corpus`.
#[derive(Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Word<'t, L>{
  /// A slice of characters in a `Corpus`.
  chars: &'t [u8],
  /// Stores the language parameter `L` of this word.
  language: PhantomData<L>
}

impl<'t, L> From<&'t[u8]> for Word<'t, L> {
  fn from(chars: &'t[u8]) -> Word<'t, L> {
    Word {
      chars: chars,
      language: PhantomData
    }
  }
}

/// The `Token` type represents word-tokens belonging to a `Corpus`.
/// In addition to words actually appearing in a `Corpus`, the `Token`
/// type includes variants for representing `Null` and `Unknown` words.
/// `Token`s should not be instantiated directly, but rather by
/// instantiating and interacting with a `Token`.
#[derive(Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum Token<'t, L=DefaultLanguage> {
  /// A `Word` token is a slice of characters that are actually present
  /// in some document.
  Word(Word<'t, L>),
  /// The `Null` token represents non-words.
  Null,
  /// The `Unknown` token variant represents words that do not appear
  /// in a canonical glossary. See [`unk`] for more information.
  /// [`unk`]: fn.unk.html
  Unknown
}

impl<'t, L> From<&'t[u8]> for Token<'t, L> {
  fn from(chars: &'t[u8]) -> Token<'t, L> {
    Token::Word(chars.into())
  }
}

impl<'l, L:Language> Token<'l, L>
{
  /// Consumes a token in one language, and produces the same token as
  /// if it belonged to another language.
  pub fn loan<M: Language>(self) -> Token<'l, M> {
    use std::mem::transmute;
    unsafe{transmute(self)}
  }
}

impl<'t,L> fmt::Debug for Word<'t,L> {
  fn fmt(&self, f: &mut fmt::Formatter) -> ::std::fmt::Result {
    use std::iter::FromIterator;
    write!(f, "{}", String::from_iter(self.chars.iter().map(|&c| c as char)))
  }
}

impl<'t,L> fmt::Display for Word<'t,L> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use std::iter::FromIterator;
    write!(f, "{}", String::from_iter(self.chars.iter().map(|&c| c as char)))
  }
}

impl<'t,L> fmt::Debug for Token<'t,L> {
  fn fmt(&self, f: &mut fmt::Formatter) -> ::std::fmt::Result {
    match *self {
      Token::Null => write!(f, "ε"),
      Token::Unknown => write!(f, "�"),
      Token::Word(ref word) => word.fmt(f)
    }
  }
}

impl<'t,L> fmt::Display for Token<'t,L> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Token::Null => write!(f, "ε"),
      Token::Unknown => write!(f, "�"),
      Token::Word(ref word) => word.fmt(f)
    }
  }
}
