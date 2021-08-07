#! /usr/bin/python3
test = "label"
result = ""
for c in test:
    result = result + chr(ord(c) ^ 13)
print(result)
