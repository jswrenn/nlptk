use std::hash::Hash;

pub trait Language
  : Sync 
  + Clone 
  + Copy 
  + Hash 
  + PartialEq 
  + PartialOrd 
  + Eq 
  + Ord  {}

/// Declares a new language type to be used as the phantom type 
/// parameter for corpora and tokens. This:
///
/// ```rust
/// language!(English);
/// ```
///
/// is expanded to:
///
/// ```rust 
/// #[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
/// pub struct English;
/// impl Language for English {}
/// ```
///
#[macro_export]
macro_rules! language {
  // `()` indicates that the macro takes no argument.
  ($l:ident) => (
      #[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
      pub struct $l;
      impl Language for $l {}
    );
}
