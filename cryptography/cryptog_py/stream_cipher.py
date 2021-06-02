import random


class KeyStream:
    def __init__(self, key=1):
        self.next = key

    def rand(self):
        self.next = (1103515245 * self.next + 12345) % 2 ** 31
        return self.next

    def get_key_byte(self):
        return self.rand() % 256


def encrypt(key_stream, message):
    return bytes([message[i] ^ key_stream.get_key_byte() for i in range(len(message))])


def run_test():
    message = "You Are Awesome"
    message = message.encode()
    key_stream = KeyStream(10)
    cipher = encrypt(key_stream, message)

    key_stream = KeyStream(10)
    decrypted = encrypt(key_stream, cipher)
    print(f"{message.decode()} -> {cipher} -> {decrypted.decode()}")
    assert message == decrypted
