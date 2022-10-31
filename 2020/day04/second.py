INPUT_FILE = "./puzzle.txt"

# Rules throw errors if they are invalid
def byr_rule(byr):
    assert(len(byr) == 4)
    assert(1920 <= int(byr) <= 2002)

def iyr_rule(iyr):
    assert(len(iyr) == 4)
    assert(2010 <= int(iyr) <= 2020)

def eyr_rule(eyr):
    assert(len(eyr) == 4)
    assert(2020 <= int(eyr) <= 2030)

def hgt_rule(hgt: str):
    assert(hgt.endswith("cm") or hgt.endswith("in"))
    unit = "cm" if hgt.endswith("cm") else "in"
    val = int(hgt.replace(unit, ""))

    if unit == "cm":
        assert(150 <= val <= 193)
    else:
        assert(59 <= val <= 76)

def hcl_rule(hcl: str):
    assert(hcl.startswith("#"))
    val = hcl[1:]
    assert(len(val) == 6)
    for ch in val:
        assert(ch in '0123456789abcdef')

def ecl_rule(ecl: str):
    assert(ecl in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"])

def pid_rule(pid: str):
    assert(len(pid) == 9)
    int(pid)

RULES = [
    ("byr", byr_rule),
    ("iyr", iyr_rule),
    ("eyr", eyr_rule),
    ("hgt", hgt_rule),
    ("hcl", hcl_rule),
    ("ecl", ecl_rule),
    ("pid", pid_rule)
]

def is_passport_valid(passport: dict) -> bool:
    try:
        for key, rule in RULES:
            rule(passport[key])
    except Exception as e:
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

            if is_passport_valid(parsed_passport):
                valid_passports += 1

    print(valid_passports)

if __name__ == "__main__":
    main()