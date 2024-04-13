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


# model = genai.GenerativeModel('gemini-1.5-pro-latest')
model = genai.GenerativeModel('gemini-pro')

def get_full_code(prompt):
    messages = [
        {'role':'user',
        'parts': [prompt]}
    ]
    response = model.generate_content(messages)
    # print(response.text)
    data = response.text
    # Remove the leading ``` and everything on that line
    code_begin = data.find("```")
    end_of_that_line = data.find("\n", code_begin)
    data = data[end_of_that_line+1:]

    # data might have the trailing ```, but if it doesn't, we need to keep asking for more
    while True:
        # did they make a complete response (has trailing ```)?
        if data.count("```") > 0:
            # parse
            code_end = data.find("```")
            data = data[:code_end]
            return data
                
        # remove the last line (it was probably half cut off)
        last_line = data.split("\n")[-1]
        data = data.split("\n")[:-1]
        data = "\n".join(data)

        messages.append({'role':'model',
                    'parts':[response.text]})
        messages.append({'role':'user',
                        'parts':[f"Continue from where you finished. Start by rewriting the last line you wrote (so start with {last_line})."]})
        response = model.generate_content(messages)
        # print(response.text)
        new_data = response.text
        
        # Remove the leading ``` and everything on that line
        code_begin = new_data.find("```")
        end_of_that_line = new_data.find("\n", code_begin)
        new_data = new_data[end_of_that_line+1:]

        data += new_data


# c_code = get_text_from_file("snake/snake.c")
# print(response.text)

# write_text_to_file("snake/snake.rs", response.text)

directory = sys.argv[1]
new_dir = os.path.join(directory,"rust_code")
# if not os.path.exists(new_dir):
#     os.makedirs(new_dir)


base_prompt = "You are an expert rust coder. You love memory safety. You will recieve a file with unsafe C code, respond with only safe rust code with identical behavior. People are counting on you to rewrite this code to safe rust. Write no extra comments.\n\nC code:\n\n"
for root, dirs, files in os.walk(directory):
    for file in files:
        if file.endswith(".c"):
            rust_code = get_full_code(base_prompt + get_text_from_file(os.path.join(root, file)))
            write_text_to_file(os.path.join(root, file.replace(".c", ".rs")), rust_code)
            print("Translated " + file + " to Rust")
