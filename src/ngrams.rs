use crate::tokens::{Token, TokenBucket};
use crate::{Histogram, WindowBuffer};

pub mod window_buffer;

/// A struct holding multiple `Histograms`
pub struct Ngrams {
    token_bucket: TokenBucket,
    histograms: Vec<Box<dyn Histo>>,
}

impl Ngrams {
    /// Create a new empty `Ngrams` for counting all the `lengths` given.
    pub fn new(lengths: impl IntoIterator<Item = usize>) -> Self {
        let histograms = lengths.into_iter().map(histo_for_len).collect();

        Self {
            token_bucket: TokenBucket::default(),
            histograms,
        }
    }

    /// Count all the occurrences of the words in `iter`
    pub fn count<'a, I: Iterator<Item = &'a str>>(&mut self, iter: I) {
        let max_len = self
            .histograms
            .iter()
            .map(|h| h.array_len())
            .max()
            .unwrap_or(1);

        WindowBuffer::new(max_len).iterate(
            iter.map(|elem| self.token_bucket.token(elem)),
            |window_buffer| {
                for histo in &mut self.histograms {
                    histo.extend_from_buffer(window_buffer);
                }
            },
        );
    }
}

fn histo_for_len(len: usize) -> Box<dyn Histo> {
    macro_rules! match_len {
        ( ( $($l:expr),* ) => $len:ident ) => {
            match $len {
                $(
                    $l => Box::new(Histogram::<[Token; $l]>::new()),
                )*
                other => Box::new((other, Histogram::<Vec<Token>>::new())),
            }
        };
    }

    match_len!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16) => len)
}

trait Histo: Send + Sync {
    fn extend_from_buffer(&mut self, word_buffer: &WindowBuffer<Token>);
    fn array_len(&self) -> usize;
}

impl<const N: usize> Histo for Histogram<[Token; N]> {
    fn extend_from_buffer(&mut self, word_buffer: &WindowBuffer<Token>) {
        self.extend_from_owned(
            word_buffer
                .windows(N)
                .map(|slice| slice.try_into().expect("slice is always N elements long")),
        );
    }

    fn array_len(&self) -> usize {
        N
    }
}

impl Histo for (usize, Histogram<Vec<Token>>) {
    #[allow(clippy::redundant_closure_for_method_calls)]
    fn extend_from_buffer(&mut self, word_buffer: &WindowBuffer<Token>) {
        self.1.extend_from_owned(
            word_buffer
                .windows(self.array_len())
                .map(|slice| slice.to_vec()),
        );
    }

    fn array_len(&self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let _ngram = Ngrams::new(1..=16);
    }
}
