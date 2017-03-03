use std::fmt;
use std::marker::PhantomData;
use language::{Language, DefaultLanguage};

#[derive(Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum Token<'t, L=DefaultLanguage> {
  /// A `Word` token is a slice of characters that are actually present
  /// in some document.
  Word {
    chars: &'t [u8],
    #[doc(hidden)]
    language: PhantomData<L>
  },
  /// The `Null` token represents non-words.
  Null,
  /// The `Unknown` token variant represents words that do not appear
  /// in a canonical glossary. See [`unk`] for more information.
  /// [`unk`]: fn.unk.html
  Unknown
}

impl<'l, L:Language> Token<'l, L>
{
  /// Consumes a token in one language, and produces the same token as
  /// if it belonged to another language.
  pub fn loan<'m, M: Language>(self) -> Token<'m, M> 
    where 'l: 'm,
  {
    use std::mem::transmute;
    unsafe{transmute(self)}
  }
}

impl<'t,L> fmt::Debug for Token<'t,L> {
  fn fmt(&self, f: &mut fmt::Formatter) -> ::std::fmt::Result {
    match *self {
      Token::Null => write!(f, "ε"),
      Token::Unknown => write!(f, "�"),
      Token::Word{chars, ..} => write!(f, "{:?}", chars)
    }
  }
}

impl<'t,L> fmt::Display for Token<'t,L> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use std::iter::FromIterator;
    match *self {
      Token::Null => write!(f, "ε"),
      Token::Unknown => write!(f, "�"),
      Token::Word{chars, ..} => write!(f, "{}",
        String::from_iter(chars.iter().map(|&c| c as char)))
    }
  }
}
