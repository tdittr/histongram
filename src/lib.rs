#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! A small crate for counting n-grams

pub use histogram::{DefaultHashBuilder, Histogram};

mod histogram;
mod ngrams;
pub mod tokens;
