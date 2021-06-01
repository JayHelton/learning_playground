import caeser_cipher
import one_time_pad
import stream_cipher

def main ():
    print("Caesar Cipher")
    caeser_cipher.run_test()
    print("One-Time Pad")
    one_time_pad.run_test()
    print("Stream Cipher")
    stream_cipher.run_test()


if __name__ == '__main__':
	main()

