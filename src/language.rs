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
/// parameter for corpora and tokens.
///
/// # Examples
/// This:
///
/// ```rust
/// language!(English);
/// ```
///
/// is expanded to this:
///
/// ```rust 
/// #[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
/// struct English;
/// impl Language for English {}
/// ```
///
/// If the language declaration is public, this macro accepts rustdoc
/// comments for the declaration. For example, this:
///
/// ```rust
/// language!(
///   /// A default language type. For projects that do not work with corpora
///   /// of different languages, it is not necessary to specify the corpora
///   /// or token type. When a language parameter is neither given nor
///   /// inferred, the type `DefaultLanguage` is used.
///   pub DefaultLanguage);
/// ```
///
/// expands to this:
///
/// ```rust
/// /// A default language type. For projects that do not work with corpora
/// /// of different languages, it is not necessary to specify the corpora
/// /// or token type. When a language parameter is neither given nor
/// /// inferred, the type `DefaultLanguage` is used.
/// pub struct DefaultLanguage;
/// impl Language for DefaultLanguage {}
/// ```
///
#[macro_export]
macro_rules! language {
  ($(#[$attr:meta])* pub $l:ident) => (
      #[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
      $(#[$attr])*
      pub struct $l;
      impl Language for $l {}
    );
  ($l:ident) => (
      #[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
      struct $l;
      impl Language for $l {}
    );
}

language!{
  /// A default language type. For projects that do not work with corpora
  /// of different languages, it is not necessary to specify the corpora
  /// or token type. When a language parameter is neither given nor
  /// inferred, the type `DefaultLanguage` is used.
  pub DefaultLanguage
}
