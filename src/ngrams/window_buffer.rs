use std::cmp::{max, min};
use std::mem::size_of;

/// A little helper to iterate over a long sequence while regularly taking windows from a buffer
#[derive(Debug)]
pub struct WindowBuffer<T> {
    buffer: Vec<T>,
    capacity: usize,
    max_len: usize,
}

impl<T> WindowBuffer<T> {
    /// Create a new Buffer of at last 4KB
    #[allow(clippy::missing_panics_doc)] // This can not panic as we clamp with max()
    #[must_use]
    pub fn new(max_len: usize) -> Self {
        Self::with_capacity(max_len, max(4096 / size_of::<T>(), 2 * max_len)).unwrap()
    }

    /// Creates a Buffer with `capacity` elements space
    ///
    /// Returns `None` if capacity is smaller than `2*max_len`.
    #[must_use]
    pub fn with_capacity(max_len: usize, capacity: usize) -> Option<Self> {
        if capacity < 2 * max_len {
            return None;
        }

        Some(Self {
            buffer: Vec::with_capacity(capacity),
            capacity,
            max_len,
        })
    }

    /// Clear the current contents of the buffer, leaving only the overflow, than fill from `iter`.
    pub fn refill(&mut self, iter: &mut impl Iterator<Item = T>) {
        self.consume();
        self.fill(iter);
    }

    /// Fill the buffer from `iter` until `capacity` is reached.
    fn fill(&mut self, iter: &mut impl Iterator<Item = T>) {
        self.buffer
            .extend(iter.take(self.capacity - self.buffer.len()));
    }

    /// Remove all the elements from the buffer placing the overflow at the start.
    fn consume(&mut self) {
        self.buffer.drain(..self.first_in_overflow(1));
    }

    /// Iterate over all the windows of length `len`.
    ///
    /// # Panics
    /// Panics if `len` is bigger than the `max_len` specified on creation.
    pub fn windows(&self, len: usize) -> impl Iterator<Item = &[T]> {
        assert!(len <= self.max_len);

        self.buffer[..self.first_in_overflow(len)].windows(len)
    }

    /// Iterate over all the elements in `iter` providing windows views into it as if you iterated
    /// over `iter` in one go with `.windows()`.
    ///
    /// # Example
    /// ```rust
    /// use histongram::WindowBuffer;
    ///
    /// let text = "The quick brown fox jumps over the lazy dog";
    ///
    /// let mut words = vec![];
    /// let mut pairs = vec![];
    /// let mut quint = vec![];
    ///
    /// WindowBuffer::new(5).iterate(text.split_whitespace(), |buf| {
    ///     let to_vec = |words: &[&'static str]| words.to_vec();
    ///
    ///     words.extend(buf.windows(1).map(to_vec));
    ///     pairs.extend(buf.windows(2).map(to_vec));
    ///     quint.extend(buf.windows(5).map(to_vec));
    /// });
    ///
    /// assert_eq!(pairs[0], ["The", "quick"]);
    /// assert_eq!(quint[4], ["jumps", "over", "the", "lazy", "dog"]);
    /// ```
    pub fn iterate<I>(mut self, mut iter: I, mut f: impl FnMut(&Self))
    where
        I: Iterator<Item = T>,
    {
        loop {
            self.refill(&mut iter);

            if self.buffer.is_empty() {
                break;
            }

            f(&self);
        }
    }

    #[inline]
    fn first_in_overflow(&self, for_len: usize) -> usize {
        let begin_of_overflow = self.capacity - self.max_len;
        let first_after_for_len = begin_of_overflow + for_len;
        min(first_after_for_len, self.buffer.len())
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn last_in_buffer() {
        let mut wb = WindowBuffer::with_capacity(3, 6).unwrap();
        wb.fill(&mut (0..));

        assert_eq!(wb.first_in_overflow(3), 6);
        assert_eq!(wb.first_in_overflow(2), 5);
        assert_eq!(wb.first_in_overflow(1), 4);

        wb.consume();
        assert_eq!(wb.first_in_overflow(3), 2);
        assert_eq!(wb.first_in_overflow(2), 2);
        assert_eq!(wb.first_in_overflow(1), 2);
    }

    #[test]
    fn basic_ops() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut input_iter = input.iter();
        let mut wb = WindowBuffer::with_capacity(3, 6).unwrap();
        wb.refill(&mut input_iter);

        {
            let mut w = wb.windows(1);
            assert_eq!(w.next(), Some([&1].as_slice()));
            assert_eq!(w.next(), Some([&2].as_slice()));
            assert_eq!(w.next(), Some([&3].as_slice()));
            assert_eq!(w.next(), Some([&4].as_slice()));
            assert_eq!(w.next(), None);

            let mut w = wb.windows(3);
            assert_eq!(w.next(), Some([&1, &2, &3].as_slice()));
            assert_eq!(w.next(), Some([&2, &3, &4].as_slice()));
            assert_eq!(w.next(), Some([&3, &4, &5].as_slice()));
            assert_eq!(w.next(), Some([&4, &5, &6].as_slice()));
            assert_eq!(w.next(), None);
        }

        wb.refill(&mut input_iter);

        {
            let mut w = wb.windows(1);
            assert_eq!(w.next(), Some([&5].as_slice()));
            assert_eq!(w.next(), Some([&6].as_slice()));
            assert_eq!(w.next(), Some([&7].as_slice()));
            assert_eq!(w.next(), Some([&8].as_slice()));
            assert_eq!(w.next(), None);

            let mut w = wb.windows(2);
            assert_eq!(w.next(), Some([&5, &6].as_slice()));
            assert_eq!(w.next(), Some([&6, &7].as_slice()));
            assert_eq!(w.next(), Some([&7, &8].as_slice()));
            assert_eq!(w.next(), None);

            let mut w = wb.windows(3);
            assert_eq!(w.next(), Some([&5, &6, &7].as_slice()));
            assert_eq!(w.next(), Some([&6, &7, &8].as_slice()));
            assert_eq!(w.next(), None);
        }

        wb.refill(&mut input_iter);

        {
            let mut w = wb.windows(1);
            assert_eq!(w.next(), None);

            let mut w = wb.windows(2);
            assert_eq!(w.next(), None);

            let mut w = wb.windows(3);
            assert_eq!(w.next(), None);
        }
    }

    proptest! {
        #[test]
        fn random_cases(input_size in 0..128usize,
                        max_len    in 1..128usize,
                        capacity   in 0..128usize,
                        len        in 1..128usize) {
            let len = min(len, max_len);
            let capacity = max(2*max_len, capacity);

            is_eq_to_windowing_once(len, input_size, max_len, capacity);
        }
    }

    #[allow(clippy::redundant_closure_for_method_calls)] // This seems to be a false positive
    fn is_eq_to_windowing_once(len: usize, input_size: usize, max_len: usize, capacity: usize) {
        let to_vec = |s: &[usize]| s.to_vec();

        let input: Vec<_> = (0..input_size).collect();
        let should = input.windows(len).map(to_vec).collect::<Vec<_>>();

        let mut output = Vec::with_capacity(input.len());

        WindowBuffer::with_capacity(max_len, capacity)
            .unwrap()
            .iterate(input.into_iter(), |wb| {
                output.extend(wb.windows(len).map(to_vec));
            });

        assert_eq!(should, output);
    }
}
