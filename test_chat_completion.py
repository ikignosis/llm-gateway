import os
from openai import OpenAI
client = OpenAI(
    base_url="http://localhost:3456/v1",
    api_key=os.environ.get("OPENAI_API_KEY"),  # This is the default and can be omitted
)
chat_completion = client.chat.completions.create(
    model="gpt-4o",
    messages=[
        {"role": "user", "content": "What is the capital of France?"},
    ],
)
print(chat_completion)