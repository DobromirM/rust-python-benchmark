import rust_python
from math import sin, cos, sqrt


def calculate():
    product = 1.0
    for c1 in range(1, 1000, 1):
        for c2 in range(1, 1000, 1):
            product = product + (c1 * c2 - sin(c1) + cos(c2) * sqrt(c1))
    return product


def test_pure_python(benchmark):
    benchmark(calculate)


def test_rust(benchmark):
    benchmark(rust_python.calculate)
