import regex

with open("puzzle.txt", 'r') as pfile:
    raw_rules, messages = pfile.read().split("\n\n")


def regexify(val, rules):
    if val.isdigit():
        return "(?:" + "".join(map(lambda x: regexify(x, rules), rules[val].split(" "))) + ")"
    else:
        return val

def parse(rules, messages):
    result = regex.compile(regexify("0", rules))
    
    print(sum(bool(result.fullmatch(m)) for m in messages.splitlines()))



rules = dict(
    raw_rule.replace('"', "").split(": ", 1)
    for raw_rule in raw_rules.splitlines()
)

rules["8"] = "42 +"
rules["11"] = "(?P<R> 42 (?&R)? 31 )"

parse(rules, messages)