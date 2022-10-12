//! Structures for turning strings into tokens that can easily be copied

use compact_str::CompactString;
use hashbrown::HashMap;
use std::num::NonZeroU32;
use std::sync::atomic::{AtomicU32, Ordering};

type TokenId = NonZeroU32;

// TODO: Make this optional with feature flag
static NEXT_BUCKET_ID: AtomicU32 = AtomicU32::new(1);

fn new_bucket_id() -> NonZeroU32 {
    NEXT_BUCKET_ID
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |old| {
            old.checked_add(1)
        })
        .expect("There will never be more that 2^32-1 TokenBuckets!")
        .try_into()
        .expect("NEXT_BUCKET_ID starts out at 1 and is thus non zero")
}

/// A basic string interner turning strings into tokens
#[derive(Debug)]
pub struct TokenBucket {
    map: HashMap<CompactString, TokenId>,
    bucket_id: NonZeroU32,
}

impl TokenBucket {
    /// Create a new empty `TokenBucket`
    ///
    /// This creates a new unique id for this bucket which will ensure that tokens from one bucket
    /// are not used with another bucket.
    pub fn new() -> Self {
        Self {
            map: Default::default(),
            bucket_id: new_bucket_id(),
        }
    }

    /// Get a [`Token`] for a given word
    ///
    /// Will always return an equal [`Token`] for an equal `word`.
    pub fn token(&mut self, word: &str) -> Token {
        let len = self.len();
        let id = self
            .map
            .entry_ref(word)
            .or_insert_with(|| Self::next_id(len));

        Token {
            id_in_bucket: *id,
            bucket_id: self.bucket_id,
        }
    }

    /// Look up the `word` that crated a [`Token`]
    ///
    /// This lookup is implemented as a linear search and thus has a complexity of `O(self.len())`.
    pub fn word(&self, token: Token) -> &str {
        assert_eq!(
            self.bucket_id, token.bucket_id,
            "Only Tokens from the same bucket may be compared!"
        );

        self.map
            .iter()
            .find_map(|(s, &t)| {
                if token.id_in_bucket == t {
                    Some(s.as_str())
                } else {
                    None
                }
            })
            .expect("There is an entry in map for every token we gave out")
    }

    /// Return the current number of unique [`Token`]s created
    pub fn len(&self) -> usize {
        self.map.len()
    }

    fn next_id(current_len: usize) -> TokenId {
        u32::try_from(current_len + 1)
            .expect("There will never be more than 2^32-1 tokens in a single bucket")
            .try_into()
            .expect("next_id+1 will always be non zero")
    }
}

impl Default for TokenBucket {
    fn default() -> Self {
        Self::new()
    }
}

/// A token pointing to a value in a [`TokenBucket`]
///
/// The toeken is only valid for the [`TokenBucket`] that created it.
///
/// # Panic
/// Comparing two `Token`s from different [`TokenBucket`]s will cause a panic.
#[derive(Copy, Clone, Eq, Hash, Debug)]
pub struct Token {
    id_in_bucket: TokenId,
    bucket_id: NonZeroU32,
}

/// # Panic
/// Comparing two `Token`s from different [`TokenBucket`]s will cause a panic.
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(
            self.bucket_id, other.bucket_id,
            "Only Tokens from the same bucket may be compared!"
        );

        self.id_in_bucket == other.id_in_bucket
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_insert() {
        let mut b = TokenBucket::new();

        let a = b.token("a");
        let aa = b.token("a");
        let c = b.token("c");
        let cc = b.token("c");

        assert_eq!(a, aa);
        assert_eq!(c, cc);

        assert_ne!(a, c);
        assert_ne!(a, cc);
        assert_ne!(aa, c);
        assert_ne!(aa, cc);

        assert_eq!(b.word(a), "a");
        assert_eq!(b.word(c), "c");
    }
}
