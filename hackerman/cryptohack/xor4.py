#! /usr/bin/python3
from pwn import xor

encrypted_msg = bytes.fromhex(
    "0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104"
)

flag_format = b"crypto{"
key = xor(encrypted_msg[:7], flag_format) + xor(encrypted_msg[-1], "}")
print(key)
flag = xor(key, encrypted_msg)
print(flag)
