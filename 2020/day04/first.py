INPUT_FILE = "./puzzle.txt"

REQURIED_FIELDS = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid"
]

def is_passport_valid(passport: dict) -> bool:
    for field in REQURIED_FIELDS:
        if field not in passport:
            return False
    
    return True

def main():
    with open(INPUT_FILE, "r", encoding="utf8") as infile:
        passports_raw = infile.read().split("\n\n")

        valid_passports = 0

        for passport in passports_raw:
            fields = passport.strip().replace("\n", " ").split(" ")

            parsed_passport = {}
            for field in fields:
                key, value = field.split(":")
                parsed_passport[key] = value

            if is_passport_valid(passport):
                valid_passports += 1

    print(valid_passports)

if __name__ == "__main__":
    main()