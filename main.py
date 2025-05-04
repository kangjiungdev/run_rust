import random


text = ""
if random.randrange(1, 3) == 1:
    text = "연산"
else:
    text = "넘어가기"

print(text)