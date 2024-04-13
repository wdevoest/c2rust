import subprocess
import sys
import os


if len(sys.argv) < 2:
    print("Usage: python file.py <executable>")
    sys.exit(1)

executable = sys.argv[1]

# check if the file exists
try:
    with open(executable, 'r') as f:
        pass
except FileNotFoundError:
    print(f"File '{executable}' not found in the current directory {os.getcwd()}")
    sys.exit(1)

# Define the input data in bytes
input_data = b'Alex' + (b'\x00'*6) + b'A+\x00'

# Run the C program using subprocess
process = subprocess.Popen([f"./{executable}"], stdin=subprocess.PIPE, stdout=subprocess.PIPE)

# Write the input data to the subprocess' stdin
process.stdin.write(input_data)
process.stdin.close()

# Read the output from the subprocess
output = process.stdout.read()

# Print or process the output as needed
print(output.decode())
