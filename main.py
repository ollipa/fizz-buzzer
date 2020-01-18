import hashlib
import time

from fizz_buzzer import fizz_buzz

def main():
    n = 10_000_000

    # Call Rust library
    start = time.time()
    result_rust = fizz_buzz(n)
    time_rust = time.time() - start

    # Call Python implementation
    start = time.time()
    result_python = fizz_my_buzz(n)
    time_python = time.time() - start

    print(f"Rust:\t{time_rust:.2f}s\t({result_rust[:10]}...)\n"
          f"Python:\t{time_python:.2f}s\t({result_python[:10]}...)")

def fizz_my_buzz(n: int):
    res = ""
    for i in range(0, n):
        if i % 15 == 0:
            res += "FizzBuzz"
        elif i % 3 == 0:
            res += "Fizz"
        elif i % 5 == 0:
            res += "Buzz"
        else:
            res += str(i)
    return hash_(res)

def hash_(data: str) -> str:
    return hashlib.sha256(bytes(data, "utf-8")).hexdigest()

main()