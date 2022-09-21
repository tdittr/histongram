use histongram::Histogram;
use std::collections::hash_map::RandomState;

#[test]
fn simple() {
    let mut h = Histogram::new();
    assert_eq!(h.num_categories(), 0);
    assert_eq!(h.num_instances(), 0);

    h.add("a");
    assert_eq!(h.num_categories(), 1);
    assert_eq!(h.num_instances(), 1);

    h.add("a");
    assert_eq!(h.num_categories(), 1);
    assert_eq!(h.num_instances(), 2);

    h.add("b");
    assert_eq!(h.num_categories(), 2);
    assert_eq!(h.num_instances(), 3);

    assert_eq!(h.count(&"a"), 2);
    assert_eq!(h.count(&"b"), 1);
    assert_eq!(h.count(&"c"), 0);
}

#[test]
fn iterating() {
    let mut h: Histogram<_> = "aaabbc".chars().collect();

    assert_eq!(h.num_categories(), 3);
    assert_eq!(h.num_instances(), 6);

    for (&key, cnt) in &h {
        match key {
            'a' => assert_eq!(cnt, 3),
            'b' => assert_eq!(cnt, 2),
            'c' => assert_eq!(cnt, 1),
            _ => unreachable!(),
        }
    }

    for (&key, cnt) in h.iter() {
        match key {
            'a' => assert_eq!(cnt, 3),
            'b' => assert_eq!(cnt, 2),
            'c' => assert_eq!(cnt, 1),
            _ => unreachable!(),
        }
    }

    for (&key, cnt) in h.iter_rel() {
        match key {
            'a' => assert_eq!(cnt, 3.0 / 6.0),
            'b' => assert_eq!(cnt, 2.0 / 6.0),
            'c' => assert_eq!(cnt, 1.0 / 6.0),
            _ => unreachable!(),
        }
    }

    h.extend("abc".chars());
    assert_eq!(h.num_categories(), 3);
    assert_eq!(h.num_instances(), 9);

    for (key, cnt) in h {
        match key {
            'a' => assert_eq!(cnt, 4),
            'b' => assert_eq!(cnt, 3),
            'c' => assert_eq!(cnt, 2),
            _ => unreachable!(),
        }
    }
}

#[test]
fn all_the_counts() {
    let h: Histogram<_> = "aaabbc".chars().collect();
    assert_eq!(h.sorted_occurrences(), vec![('a', 3), ('b', 2), ('c', 1)]);
}

#[test]
fn appending() {
    let mut hist = ["a", "a", "b"].into_iter().collect::<Histogram<_>>();

    assert_eq!(hist.count(&"a"), 2);
    assert_eq!(hist.count(&"b"), 1);

    hist.append(hist.clone());

    assert_eq!(hist.count(&"a"), 4);
    assert_eq!(hist.count(&"b"), 2);
    assert_eq!(hist.count(&"c"), 0);

    let other = Histogram::from_iter(["c"; 15]);

    hist.append(other);
    assert_eq!(hist.count(&"c"), 15);
}

#[test]
fn strong_hash() {
    let mut h = Histogram::with_hasher(RandomState::new());
    h.add("foo");
    assert_eq!(h.count(&"foo"), 1);
}
