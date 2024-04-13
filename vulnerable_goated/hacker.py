import subprocess
import sys
# Define the input data in bytes
input_data = (b'\x00'*20)
# Run the C program using subprocess
process = subprocess.Popen(['./vulnerable'], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
# Write the input data to the subprocess' stdin
process.stdin.write(input_data)
process.stdin.close()
# Read the output from the subprocess
output = process.stdout.read()
# Print or process the output as needed
print(output.decode())