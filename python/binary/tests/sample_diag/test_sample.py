from sample_diag import sample


def test_compute():
    assert sample.compute(2) == 4, "incorrect compute result"
