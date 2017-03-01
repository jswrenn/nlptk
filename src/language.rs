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

#[macro_export]
macro_rules! language {
  // `()` indicates that the macro takes no argument.
  ($l:ident) => (
      #[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
      pub struct $l;
      impl Language for $l {}
    );
}
