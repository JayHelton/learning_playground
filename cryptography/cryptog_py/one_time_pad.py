import random

def generate_key_stream(n):
    return bytes([random.randrange(0, 256) for i in range(n)])

def xor_bytes(key_stream, message):
    length = min(len(key_stream), len(message))
    return bytes([key_stream[i] ^ message[i] for i in range(length)])

def run_test():
    message = "You Are Awesome"
    message = message.encode()
    key_stream = generate_key_stream(len(message))
    cipher = xor_bytes(key_stream, message)
    decrypted = xor_bytes(key_stream, cipher)
    print(f'{message.decode()} -> {cipher} -> {decrypted.decode()}')
    assert(message == decrypted)
