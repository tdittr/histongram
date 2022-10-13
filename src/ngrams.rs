use multi_token_histogram::MultiLenTokenHistoNgram;

use crate::tokens::TokenBucket;
use crate::WindowBuffer;

mod multi_token_histogram;
pub mod window_buffer;

/// A struct holding multiple `Histograms`
pub struct Ngrams {
    token_bucket: TokenBucket,
    histograms: Vec<MultiLenTokenHistoNgram>,
}

impl Ngrams {
    /// Create a new empty `Ngrams` for counting all the `lengths` given.
    pub fn new(lengths: impl IntoIterator<Item = usize>) -> Self {
        let histograms = lengths
            .into_iter()
            .map(MultiLenTokenHistoNgram::new)
            .collect();

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
            .map(MultiLenTokenHistoNgram::array_len)
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
