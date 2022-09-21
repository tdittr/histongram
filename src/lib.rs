#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use std::cmp::Reverse;
use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter;

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
/// This can be build by using Iterators or using [sorted_occurences].
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
pub struct Histogram<K: Hash + Eq> {
    map: HashMap<K, usize>,
}

impl<K: Hash + Eq> Histogram<K> {
    /// Create a new empty `Histogram`
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
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
        let total = self.num_instances() as f64;
        self.iter().map(move |(k, cnt)| (k, cnt as f64 / total))
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
        counts.sort_by_key(|(_key, cnt)| Reverse(*cnt));
        counts
    }
}

// This can not be derived as it would then only be available if `K: Default` which we don't need here.
impl<K: Hash + Eq> Default for Histogram<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Hash + Eq> Extend<K> for Histogram<K> {
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        for item in iter {
            self.add(item);
        }
    }
}

impl<K: Hash + Eq> FromIterator<K> for Histogram<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut h = Self::new();
        h.extend(iter);
        h
    }
}

impl<'a, K: Hash + Eq + 'a> IntoIterator for &'a Histogram<K> {
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

impl<K: Hash + Eq> IntoIterator for Histogram<K> {
    type Item = (K, usize);
    type IntoIter = hash_map::IntoIter<K, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}
