use std::cmp::{min, Reverse};

use insta::assert_ron_snapshot;

use histongram::Histogram;

const APACHE: &str = include_str!("../LICENSE-APACHE");
const MIT: &str = include_str!("../LICENSE-MIT");

#[test]
fn license_words() {
    let mut a = Histogram::from_iter(APACHE.split_whitespace());
    let mut a_counts = a.clone().sorted_occurrences();
    sort_also_by_key(&mut a_counts);
    assert_ron_snapshot!(a_counts);

    let m = Histogram::from_iter(MIT.split_whitespace());
    let mut m_counts = m.clone().sorted_occurrences();
    sort_also_by_key(&mut m_counts);
    assert_ron_snapshot!(m_counts);

    a.append(m);
    let mut combined_counts = a.clone().sorted_occurrences();
    sort_also_by_key(&mut combined_counts);
    assert_ron_snapshot!(combined_counts);
}

#[test]
fn license_chars() {
    let mut a = Histogram::from_iter(APACHE.chars());
    let mut a_counts = a.clone().sorted_occurrences();
    sort_also_by_key(&mut a_counts);
    assert_ron_snapshot!(a_counts);

    let m = Histogram::from_iter(MIT.chars());
    let mut m_counts = m.clone().sorted_occurrences();
    sort_also_by_key(&mut m_counts);
    assert_ron_snapshot!(m_counts);

    a.append(m);
    let mut combined_counts = a.clone().sorted_occurrences();
    sort_also_by_key(&mut combined_counts);
    assert_ron_snapshot!(combined_counts);
}

fn sort_also_by_key<K: Ord + Copy>(counts: &mut [(K, usize)]) {
    // This table should be sorted by count. But items with the same number of occurrences are
    // in arbitrary order...
    let mut high = usize::MAX;
    for &(_, cnt) in counts.iter() {
        assert!(cnt <= high);
        high = min(high, cnt);
    }

    // Now sort also by key so the snapshot stays consistent
    counts.sort_unstable_by_key(|(key, cnt)| (Reverse(*cnt), *key));
}
