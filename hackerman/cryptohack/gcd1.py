#! /usr/bin/python3


def gcd(a, b):
    if a == 0:
        return b
    # modulus works better than looping a subtraction until a is less than
    # b
    return gcd(b % a, a)


a = 66528
b = 52920

print(gcd(a, b))
