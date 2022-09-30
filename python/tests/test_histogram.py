from histongram import Histogram


def test_new():
    h = Histogram()

    h.add("foo")
    assert h["foo"] == 1

    h.add_many(["foo", "foo"])
    assert h["foo"] == 3

    h.add_many("bar bar bar".split(" "))
    assert h["bar"] == 3

    h.add_many(iter("nope"))
    assert h["n"] == 1

    h.add_many(c for c in "nope" if c != "o")
    assert h["n"] == 2
    assert h["o"] == 1
