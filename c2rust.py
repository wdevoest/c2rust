import os
import google.generativeai as genai

GOOGLE_API_KEY=os.environ['GOOGLE_API_KEY']
genai.configure(api_key=GOOGLE_API_KEY)


prompt = """This image contains a sketch of a potential product along with some notes.
Given the product sketch, describe the product as thoroughly as possible based on what you
see in the image, making sure to note all of the product features. Return output in json format:
{description: description, features: }"""


model = genai.GenerativeModel('gemini-pro')
response = model.generate_content(prompt)
print(response.text)