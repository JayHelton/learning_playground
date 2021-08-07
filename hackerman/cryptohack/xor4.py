#! /usr/bin/python3
from pwn import xor

encrypted_msg = "0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104"
encrypted_msg = bytes.fromhex(encrypted_msg)

flag_format = b"crypto{"

key = [o1 ^ o2 for (o1, o2) in zip(encrypted_msg, flag_format)] + [ord("y")]

key_len = len(key)
flag = xor(bytes(key), encrypted_msg)
print("Flag:")
print(flag)
