import math
import random


def is_prime(p):
    for i in range(2, math.isqrt(p) + 1):
        if p % i == 0:
            return False
    return True


def get_prime(size):
    while True:
        p = random.randrange(size, 2 * size)
        if is_prime(p):
            return p


def lcm(a, b):
    return a * b // math.gcd(a, b)


def get_e(lambda_n):
    for e in range(2, lambda_n):
        if math.gcd(e, lambda_n) == 1:
            return e
    return False


def run_tests():
    size = 300
    # 1: Generate two distinct primes
    p = get_prime(size)
    q = get_prime(size)
    print(f"generated primes: {p} - {q}")
    # 2: compute n = p*q
    n = p * q
    print(f"modules n: {n}")
    # 3: compute lambda alg
    lambda_n = lcm(p - 1, q - 1)
    # 4: choose an int
    e = get_e(lambda_n)
