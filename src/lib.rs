#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! A small crate for counting n-grams

extern crate core;

pub use histogram::{DefaultHashBuilder, Histogram};
pub use ngrams::window_buffer::WindowBuffer;
pub use ngrams::Ngrams;

mod histogram;
mod ngrams;
pub mod tokens;
