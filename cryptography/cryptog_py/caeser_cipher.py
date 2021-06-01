def generate_key(n):
    letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    key = {}
    cnt = 0
    for c in letters:
        key[c] = letters[(cnt + n) % len(letters)]
        cnt += 1
    return key

def encrypt(key, message):
    cipher = ""
    for c in message:
        if c in key:
            cipher += key[c]
        else:
            cipher += c
    return cipher

def generate_decrypt_key(key):
    dkey = {}
    for c in key:
        dkey[key[c]] = c
    return dkey;

def run_test():
    key = generate_key(3)
    message = "YOU ARE AWESOME"
    cipher = encrypt(key, message)
    dkey = generate_decrypt_key(key)
    decrypted = encrypt(dkey, cipher)
    assert(message == decrypted)
    print(message, "->", cipher, "->", decrypted)
