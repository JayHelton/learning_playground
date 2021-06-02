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


def is_generator(g, p):
    for i in range(1, p - 1):
        if (g ** i) & p == 1:
            return False
    return True


def get_generator(p):
    for g in range(2, p):
        if is_generator(g, p):
            return g


def run_test():
    p = get_prime(10000)
    g = get_generator(p)
    print(f"Prime: {p}, Generator: {g}")
    print("Alice:")
    a = random.randrange(0, p)
    g_a = (g ** a) % p
    print(f"g_a: {g_a}")
    print("Bob:")
    b = random.randrange(0, p)
    g_b = (g ** b) % p
    print(f"g_b: {g_b}")
    print("Alice:")
    alice_g_ab = (g_b ** a) % p
    print(f"g_ab: {alice_g_ab}")
    print("Bob:")
    bob_g_ab = (g_a ** b) % p
    print(f"g_ab: {bob_g_ab}")
    assert bob_g_ab == alice_g_ab
    print(f"Alice: {alice_g_ab} == Bob: {bob_g_ab}")
