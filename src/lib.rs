#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use std::cmp::Reverse;
use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::iter;

#[cfg(feature = "fxhash")]
type DefaultHasher = fxhash::FxBuildHasher;
#[cfg(not(feature = "fxhash"))]
type DefaultHasher = hash_map::RandomState;

/// A histogram that counts occurrences of `key`s.
///
/// # Examples
///
/// ## Add and get one by one
/// ```rust
/// use histongram::Histogram;
///
/// let mut hist = Histogram::new();
///
/// hist.add('a');
/// assert_eq!(hist.count(&'a'), 1);
/// assert_eq!(hist.count(&'b'), 0);
/// ```
///
/// ## Filling from Iterators
/// ```rust
/// use histongram::Histogram;
/// let mut hist: Histogram<_> = ["a", "a", "a"].into_iter().collect();
///
/// assert_eq!(hist.count(&"a"), 3);
///
/// hist.extend(["a", "b", "c"]);
/// assert_eq!(hist.count(&"a"), 4);
/// assert_eq!(hist.count(&"b"), 1);
/// assert_eq!(hist.count(&"c"), 1);
/// ```
///
/// ## Iterating the counts
/// ```rust
/// use histongram::Histogram;
/// let hist: Histogram<_> = ["a", "a", "a"].into_iter().collect();
///
/// // NOTE: The order is arbitrary for multiple items
///
/// for (key, cnt) in &hist {
///     assert_eq!(key, &"a");
///     assert_eq!(cnt, 3);
/// }
///
/// for (key, portion) in hist.iter_rel() {
///     assert_eq!(key, &"a");
///     assert_eq!(portion, 1.0);
/// }
///
/// // This consumes hist but gives back ownership of the keys
/// for (key, cnt) in hist {
///     assert_eq!(key, "a");
///     assert_eq!(cnt, 3);
/// }
/// ```
///
/// ## Getting a sorted vector of occurences
/// This can be build by using Iterators or using [`Histogram::sorted_occurrences()`].
/// ```rust
/// use std::cmp::Reverse;
/// use histongram::Histogram;
/// let hist: Histogram<_> = "aaaxxzzzzz".chars().collect();
///
/// let mut counts: Vec<_> = hist.into_iter().collect();
/// counts.sort_by_key(|(_key, cnt)| Reverse(*cnt));
///
/// assert_eq!(counts, vec![
///     ('z', 5),
///     ('a', 3),
///     ('x', 2),
/// ]);
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Histogram<K: Hash + Eq, H: BuildHasher = DefaultHasher> {
    map: HashMap<K, usize, H>,
}

impl<K: Hash + Eq> Histogram<K, DefaultHasher> {
    /// Create a new empty `Histogram`
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: HashMap::default(),
        }
    }
}

impl<K: Hash + Eq, H: BuildHasher> Histogram<K, H> {
    pub fn with_hasher(hash_builder: H) -> Self {
        Self {
            map: HashMap::with_hasher(hash_builder),
        }
    }

    /// Number of categories in the histogram
    ///
    /// # Example
    /// ```rust
    /// # use histongram::Histogram;
    /// let mut hist = Histogram::new();
    ///
    /// hist.add("abc");
    /// hist.add("abc");
    /// hist.add("other");
    /// assert_eq!(hist.num_categories(), 2);
    /// ```
    #[must_use]
    pub fn num_categories(&self) -> usize {
        self.map.len()
    }

    /// Total number of instances inserted so far
    ///
    /// # Example
    /// ```rust
    /// # use histongram::Histogram;
    /// let mut hist = Histogram::new();
    ///
    /// hist.add("abc");
    /// hist.add("abc");
    /// hist.add("other");
    /// assert_eq!(hist.num_instances(), 3);
    /// ```
    #[must_use]
    pub fn num_instances(&self) -> usize {
        self.map.values().sum()
    }

    /// Add a new occurrence of `key`  
    pub fn add(&mut self, val: K) {
        let cnt = self.map.entry(val).or_default();
        *cnt += 1;
    }

    /// Add all the occurrences from `other` to self
    pub fn append(&mut self, other: Self) {
        for (key, cnt) in other {
            let old = self.map.entry(key).or_default();
            *old += cnt;
        }
    }

    /// Get the number of times `key` was added to this histogram
    ///
    /// Returns `0` for absent `key`s.
    ///
    /// # Example
    /// ```rust
    /// # use histongram::Histogram;
    /// let mut hist = Histogram::new();
    ///
    /// hist.add("present");
    /// assert_eq!(hist.count(&"present"), 1);
    /// assert_eq!(hist.count(&"absent"), 0);
    /// ```
    pub fn count(&self, key: &K) -> usize {
        self.map.get(key).copied().unwrap_or(0)
    }

    /// Get the relative number of times `key` was added to this histogram
    ///
    /// Returns `0.0` for absent `key`s, so also if asked for any key in an empty `Histogram`.
    /// If all occurrences so far matched `key` `count_rel` will return `1.0`.
    ///
    /// # Example
    /// ```rust
    /// # use histongram::Histogram;
    /// let mut hist = Histogram::new();
    ///
    /// hist.add("present");
    /// assert_eq!(hist.count_rel(&"present"), 1.0);
    /// assert_eq!(hist.count_rel(&"absent"), 0.0);
    /// ```
    pub fn count_rel(&self, key: &K) -> f64 {
        let total = self.num_instances();
        if total == 0 {
            // There are no instances, so `key` can also not be in the list
            // And 0% seem reasonable for an absent key
            return 0.0;
        }

        // Rounding is fine when the numbers get to large to fit f64
        #[allow(clippy::cast_precision_loss)]
        {
            self.count(key) as f64 / total as f64
        }
    }

    /// Iterate over all `key`s and their counts in `self` that have occurred at least once.
    ///
    /// The order of keys is arbitrary.
    pub fn iter(&self) -> impl Iterator<Item = (&K, usize)> {
        self.into_iter()
    }

    /// Iterate over all `key`s which have occurred at least once, together with its relative
    /// number of occurrences.
    ///
    /// The order of keys is arbitrary.
    pub fn iter_rel(&self) -> impl Iterator<Item = (&K, f64)> {
        // If the counts get to big rounding is fine here.
        #[allow(clippy::cast_precision_loss)]
        {
            let total = self.num_instances() as f64;
            self.iter().map(move |(k, cnt)| (k, cnt as f64 / total))
        }
    }

    /// Get a vector of `key`s and `count`s sorted descending by `count`.
    ///
    /// This consumes the Histogram to avoid cloning the `key`s.
    ///
    /// ```rust
    /// use histongram::Histogram;
    /// let hist: Histogram<_> = "aaaxxzzzzz".chars().collect();
    ///
    /// assert_eq!(hist.sorted_occurrences(), vec![
    ///     ('z', 5),
    ///     ('a', 3),
    ///     ('x', 2),
    /// ]);
    /// ```
    #[must_use]
    pub fn sorted_occurrences(self) -> Vec<(K, usize)> {
        let mut counts: Vec<_> = self.into_iter().collect();
        // NOTE: unstable is okay here, as the map order is already arbitrary
        counts.sort_unstable_by_key(|(_key, cnt)| Reverse(*cnt));
        counts
    }
}

// This can not be derived as it would then only be available if `K: Default` which we don't need here.
impl<K: Hash + Eq, H: BuildHasher + Default> Default for Histogram<K, H> {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
        }
    }
}

impl<K: Hash + Eq, H: BuildHasher> Extend<K> for Histogram<K, H> {
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        for item in iter {
            self.add(item);
        }
    }
}

impl<K, H> FromIterator<K> for Histogram<K, H>
where
    K: Hash + Eq,
    H: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut h = Self {
            map: HashMap::with_hasher(Default::default()),
        };
        h.extend(iter);
        h
    }
}

impl<'a, K: Hash + Eq + 'a, H: BuildHasher> IntoIterator for &'a Histogram<K, H> {
    type Item = (&'a K, usize);
    type IntoIter = iter::Map<hash_map::Iter<'a, K, usize>, fn((&'a K, &'a usize)) -> Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // This can not be a closure as we need to name its type in `IntoIter`
        fn deref_cnt<'a, K>((key, cnt): (&'a K, &'a usize)) -> (&'a K, usize) {
            (key, *cnt)
        }

        self.map.iter().map(deref_cnt)
    }
}

impl<K: Hash + Eq, H: BuildHasher> IntoIterator for Histogram<K, H> {
    type Item = (K, usize);
    type IntoIter = hash_map::IntoIter<K, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<K: Hash + Eq, H: BuildHasher> From<HashMap<K, usize, H>> for Histogram<K, H> {
    fn from(map: HashMap<K, usize, H>) -> Self {
        Self { map }
    }
}

impl<K: Hash + Eq, H: BuildHasher> From<Histogram<K, H>> for HashMap<K, usize, H> {
    fn from(hist: Histogram<K, H>) -> Self {
        hist.map
    }
}

#[cfg(feature = "serde")]
mod serde {
    use std::collections::HashMap;
    use std::hash::{BuildHasher, Hash};

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::Histogram;

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<K, H> Serialize for Histogram<K, H>
    where
        K: Hash + Eq + Serialize,
        H: BuildHasher,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.map.serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de, K, H> Deserialize<'de> for Histogram<K, H>
    where
        K: Hash + Eq + Deserialize<'de>,
        H: BuildHasher + Default,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(Self {
                map: HashMap::deserialize(deserializer)?,
            })
        }
    }
}
