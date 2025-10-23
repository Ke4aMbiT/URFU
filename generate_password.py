import random
import string

def generate_password(length=8):
    characters = string.ascii_letters + string.digits + string.punctuation
    password = ''.join(random.sample(characters, length))
    return password

# Используем генератор для получения пароля длиной 12 символов
my_password = generate_password(12)
print(my_password)