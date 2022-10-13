// This aligns all the tokens
#![allow(clippy::zero_prefixed_literal)]

use std::hash::BuildHasherDefault;
use std::num::NonZeroUsize;

use crate::tokens::Token;
use crate::{Histogram, WindowBuffer};

type TokenHistoNgram<const N: usize> =
    Histogram<[Token; N], BuildHasherDefault<rustc_hash::FxHasher>>;

pub enum MultiLenTokenHistoNgram {
    Empty,
    Dyn(
        NonZeroUsize,
        Histogram<Vec<Token>, BuildHasherDefault<rustc_hash::FxHasher>>,
    ),
    F01(TokenHistoNgram<01>),
    F02(TokenHistoNgram<02>),
    F03(TokenHistoNgram<03>),
    F04(TokenHistoNgram<04>),
    F05(TokenHistoNgram<05>),
    F06(TokenHistoNgram<06>),
    F07(TokenHistoNgram<07>),
    F08(TokenHistoNgram<08>),
    F09(TokenHistoNgram<09>),
    F10(TokenHistoNgram<10>),
    F11(TokenHistoNgram<11>),
    F12(TokenHistoNgram<12>),
    F13(TokenHistoNgram<13>),
    F14(TokenHistoNgram<14>),
    F15(TokenHistoNgram<15>),
    F16(TokenHistoNgram<16>),
}

impl MultiLenTokenHistoNgram {
    #[allow(clippy::enum_glob_use)]
    pub fn new(len: usize) -> Self {
        use MultiLenTokenHistoNgram::*;
        match len {
            00 => Empty,
            01 => F01(TokenHistoNgram::new_fxhash()),
            02 => F02(TokenHistoNgram::new_fxhash()),
            03 => F03(TokenHistoNgram::new_fxhash()),
            04 => F04(TokenHistoNgram::new_fxhash()),
            05 => F05(TokenHistoNgram::new_fxhash()),
            06 => F06(TokenHistoNgram::new_fxhash()),
            07 => F07(TokenHistoNgram::new_fxhash()),
            08 => F08(TokenHistoNgram::new_fxhash()),
            09 => F09(TokenHistoNgram::new_fxhash()),
            10 => F10(TokenHistoNgram::new_fxhash()),
            11 => F11(TokenHistoNgram::new_fxhash()),
            12 => F12(TokenHistoNgram::new_fxhash()),
            13 => F13(TokenHistoNgram::new_fxhash()),
            14 => F14(TokenHistoNgram::new_fxhash()),
            15 => F15(TokenHistoNgram::new_fxhash()),
            16 => F16(TokenHistoNgram::new_fxhash()),
            other => Dyn(NonZeroUsize::new(other).unwrap(), Histogram::new_fxhash()),
        }
    }

    #[allow(clippy::enum_glob_use)]
    pub fn extend_from_buffer(&mut self, word_buffer: &WindowBuffer<Token>) {
        use MultiLenTokenHistoNgram::*;
        macro_rules! match_extend {
            ( ( $($l:ident),* ) => $self:ident, $word_buffer:ident ) => {
                let len = self.array_len();
                match $self {
                    $(
                        $l(h) => {
                            h.extend_from_owned(
                                $word_buffer
                                    .windows(len)
                                    .map(|slice| slice.try_into().expect("slice is always N elements long")),
                            );
                        },
                    )*
                    Empty => {},
                    Dyn(len, h) => {
                        h.extend_from_owned(
                            $word_buffer
                                .windows((*len).into())
                                .map(|slice| slice.to_vec()),
                        );
                    },
                }
            };
        }

        match_extend!((F01, F02, F03, F04, F05, F06, F07, F08, F09, F10, F11, F12, F13, F14, F15, F16) => self, word_buffer);
    }

    #[allow(clippy::enum_glob_use)]
    pub fn array_len(&self) -> usize {
        use crate::ngrams::MultiLenTokenHistoNgram::Empty;
        use MultiLenTokenHistoNgram::*;
        match self {
            Empty => 0,
            Dyn(len, _) => (*len).into(),
            F01(_) => 1,
            F02(_) => 2,
            F03(_) => 3,
            F04(_) => 4,
            F05(_) => 5,
            F06(_) => 6,
            F07(_) => 7,
            F08(_) => 8,
            F09(_) => 9,
            F10(_) => 10,
            F11(_) => 11,
            F12(_) => 12,
            F13(_) => 13,
            F14(_) => 14,
            F15(_) => 15,
            F16(_) => 16,
        }
    }
}
