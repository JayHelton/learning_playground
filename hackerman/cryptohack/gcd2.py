#! /usr/bin/python3


def gcd_extended(a, b):

    # Base Case
    if a == 0:
        return b, 0, 1

    gcd, x1, y1 = gcd_extended(b % a, a)

    # Update x and y using results of recursive
    # call
    x = y1 - (b // a) * x1
    y = x1

    return gcd, x, y


a, b = 35, 15
g, x, y = gcd_extended(a, b)
print("gcd(", a, ",", b, ") = ", g)
