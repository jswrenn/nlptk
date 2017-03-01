#![feature(try_from)]
#![feature(conservative_impl_trait)]
#![allow(non_snake_case)]
#[macro_use] extern crate nlptk;
extern crate itertools;
extern crate vosealias;
extern crate fnv;

use vosealias::AliasTable as Roulette;
use nlptk::*;
use std::env;
use std::fs::File;
use std::convert::TryInto;
use std::iter::FromIterator;
use std::hash;
use itertools::Itertools;
use fnv::FnvHashMap;
use std::collections::HashMap;

// Corpora and tokens are tagged with a Language type parameter. This
// prevents accidental access. 
language!(English);

/// Given an input stream of tokens, produces a mapping between each
/// word type and its frequency in the stream.
pub fn frequency<T, I, S>(ngrams: I)
    -> HashMap<T, usize, S> 
  where T: hash::Hash + Ord + Eq,
        I: IntoIterator<Item=T>,
        S: hash::BuildHasher + Default {
  let mut frequency = HashMap::<T, usize, S>::default();
  for ngram in ngrams {
    let count = frequency.entry(ngram).or_insert(0);
    *count += 1;
  }
  frequency
}

fn main() {
  // Open the first file path specified as a command line argument
  let mut files = env::args().skip(1)
    .map(File::open)
    .map(Result::unwrap)
    .take(1);

  // Construct the training corpus from that file
  let training: Corpus<English> = 
    files.next()
      .unwrap()
      .try_into()
      .unwrap();

  // Construct a lookup table mapping each observed sentence length to
  // the number of sentences of that length.
  let sentence_length_frequency : FnvHashMap<_,_> =
    frequency(training.sentences().iter().map(|n| n.len()));

  // Construct a lookup table mapping each observed word to the number
  // of times that word was observed.
  let word_frequency : FnvHashMap<_,_> =
    frequency(training.words().iter());

  // Construct a discrete probability distribution of sentence
  // lengths using the alias method.
  // https://en.wikipedia.org/wiki/Alias_method
  let sentence_length_picker =
    Roulette::from_iter(
      sentence_length_frequency.iter().map(|(l, &f)| (l, f as f64)));

  // Construct a discrete probability distribution of words using
  // the alias method.
  // https://en.wikipedia.org/wiki/Alias_method
  let word_picker =
    Roulette::from_iter(
      word_frequency.iter().map(|(w, &f)| (w, f as f64)));

  // Sample from the probability distribution of sentence lengths
  sentence_length_picker.into_iter()
    // For each sampled length `l`, sample `l` words from the
    // probability distribution of words, and join them together with
    // spaces.
    .map(|&&len| word_picker.into_iter().take(len).join(" "))
    // Print each sentence on its own line.
    .foreach(| sentence| println!("{}", sentence));
}
