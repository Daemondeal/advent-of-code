import os
import shutil
import argparse
import requests

from http.client import responses
from secret import SESSION_TOKEN

BASE_URL = ""

def write_all(path: str, content: str):
    with open(path, "w") as outfile:
        outfile.write(content)

def setup_python(dir: str, input: str):
    write_all(f"{dir}/input.txt", input)
    shutil.copy2("./templates/python.py", f"{dir}/main.py")

def setup_rust(dir: str, input: str):
    os.system(f"cargo init {dir}/ --bin --vcs none")
    write_all(f"{dir}/input.txt", input)
    os.remove(f"{dir}/src/main.rs")
    shutil.copy2("./templates/rust.rs", f"{dir}/src/main.rs")


def parse_arguments():
    parser = argparse.ArgumentParser(
        prog="setup",
        description="sets up a new Advent of Code day",
    )

    parser.add_argument("year", type=int)
    parser.add_argument("day", type=int)

    parser.add_argument("-l", "--language", choices=["rust", "python"], default="python")


    return parser.parse_args()

def main():
    args = parse_arguments()

    year = args.year
    day = args.day

    page_url = f"https://adventofcode.com/{year}/day/{day}/input"

    directory = f"./{year}/day{day:02}"

    if os.path.isdir(directory):
        print(f"Directory '{directory}' already exists")
        return

    response = requests.get(page_url, cookies={"session": SESSION_TOKEN})

    if response.status_code != 200:
        print(f"Error getting {page_url}: {response.status_code} ({responses[response.status_code]})")
        return

    input = response.text

    os.makedirs(directory, exist_ok=True)

    if args.language == "python":
        setup_python(directory, input)
    elif args.language == "rust":
        setup_rust(directory, input)
    else:
        print(f"Invalid language: {args.language}")


if __name__ == "__main__":
    main()