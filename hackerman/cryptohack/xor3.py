#! /usr/bin/python3
from pwn import xor

test = bytes.fromhex(
    "73626960647f6b206821204f21254f7d694f7624662065622127234f726927756d"
)
for i in range(0, 257):
    result = xor(test, i).decode("utf-8")
    if "crypto" in result:
        print(result)
