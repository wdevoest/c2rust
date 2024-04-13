import google.generativeai as genai
import os
import sys

def load_dotenv(dotenv_path=".env"):
    with open(dotenv_path, "r") as f:
        for line in f:
            if line.strip() and not line.startswith("#"):
                key, value = line.strip().split("=", 1)
                os.environ[key] = value

# Load environment variables from .env file
load_dotenv()
GOOGLE_API_KEY=os.getenv('GOOGLE_API_KEY')
# print(GOOGLE_API_KEY)
genai.configure(api_key=GOOGLE_API_KEY)

# Get text from a file
def get_text_from_file(file_path):
    with open(file_path, 'r') as file:
        return file.read()
def write_text_to_file(file_path, text):
    with open(file_path, 'w') as file:
        file.write(text)
def get_command_output(command, folder=None):
    if folder:
        # enter the folder first
        os.chdir(folder)
    output = os.popen(command).read()
    if folder:
        # return to the previous folder
        os.chdir('..')
    return output

def get_test_case_output(executable, input_type, input_file, folder):
        if input_type == "filein":
            return get_command_output("./" + executable + " < " + input_file, folder)
        elif input_type == "pythonin":
            return get_command_output("python3 " + input_file + " " + executable, folder)
        else:
            print("Invalid input type")
            return None


model = genai.GenerativeModel('gemini-pro')

# c_code = get_text_from_file("snake/snake.c")
# print(response.text)

# write_text_to_file("snake/snake.rs", response.text)

directory = sys.argv[1]

for root, dirs, files in os.walk(directory):
    for file in files:
        if file.endswith(".c"):
            response = model.generate_content("Translate the following C code to Rust:\n\n" + get_text_from_file(os.path.join(root, file)))
            write_text_to_file(os.path.join(root, file.replace(".c", ".rs")), response.text)
            print("Translated " + file + " to Rust")

