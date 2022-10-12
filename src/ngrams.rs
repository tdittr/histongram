use crate::tokens::{Token, TokenBucket};
use crate::Histogram;

type OptGram<const N: usize> = Option<Histogram<[Token; N]>>;

macro_rules! optgram_for_each {
     ( $( $idx:tt ),* ) => {
         (
            $(
                OptGram<$idx>,
            )*
         )
     }
}

pub struct Ngrams {
    token_bucket: TokenBucket,
    histograms: optgram_for_each!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10),
}

impl Ngrams {
    fn for_each(&mut self, mut f: impl FnMut(usize, Option<&mut dyn Histo>)) {
        macro_rules! for_each_call {
            ( ($( $idx:tt ),*) => $f:ident ) => {
                $({

                    $f(($idx + 1), (self.histograms.$idx).as_mut().map(|histo| histo as &mut dyn Histo));
                })*
            };
        }

        for_each_call!((0, 1, 2, 3, 4, 5, 6, 7, 8, 9) => f);
    }

    fn for_each_existing(&mut self, mut f: impl FnMut(&mut dyn Histo)) {
        self.for_each(|_, h| match h {
            Some(h) => f(h),
            None => (),
        })
    }

    pub fn new(for_lengths: &[usize]) -> Self {
        let new = Self {
            token_bucket: Default::default(),
            histograms: (None, None, None, None, None, None, None, None, None, None),
        };

        new
    }
}

trait Histo {
    fn add(&mut self, words: &[Token]);
    fn array_len(&self) -> usize;
}

impl<const N: usize> Histo for Histogram<[Token; N]> {
    fn add(&mut self, words: &[Token]) {
        self.add_owned(
            words
                .try_into()
                .expect("Length of words must match the value returned by `array_len`"),
        )
    }

    fn array_len(&self) -> usize {
        N
    }
}
