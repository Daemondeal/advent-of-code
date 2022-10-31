from abc import ABC
from copy import deepcopy
from typing import Optional

INPUT_FILE = "./puzzle.txt"

class BaseRule(ABC):
    def check_rule(self, text_stack):
        pass


class FinalRule(BaseRule):
    def __init__(self, subrule):
        self.subrule = subrule

    def check_rule(self, text_stack):
        success = self.subrule.check_rule(text_stack)

        return success and len(text_stack) == 0

    def __str__(self):
        return str(self.subrule)


class SuperRule(BaseRule):
    def __init__(self, subrules):
        self.subrules = subrules

    def check_rule(self, text_stack):
        for subrule in self.subrules:
            if not subrule.check_rule(text_stack):
                return False
        
        return True

    def __str__(self):
        return " ".join(map(lambda x: str(x), self.subrules))

class OptionalRule(BaseRule):
    subrule_a: BaseRule
    subrule_b: BaseRule

    def __init__(self, subrule_a, subrule_b):
        self.subrule_a = subrule_a
        self.subrule_b = subrule_b

    def check_rule(self, text_stack):
        stack_clone = deepcopy(text_stack)
        if self.subrule_a.check_rule(stack_clone):
            text_stack.clear()
            for element in stack_clone:
                text_stack.append(element)

            return True
        else:
            return self.subrule_b.check_rule(text_stack)

    def __str__(self):
        return str(self.subrule_a) + " | " + str(self.subrule_b)


class LeafRule(BaseRule):
    def __init__(self, char):
        self.char = char

    def check_rule(self, text_stack):
        # print("Checking", self.char, "against", text_stack)
        return len(text_stack) > 0 and text_stack.pop(0) == self.char

    def __str__(self):
        return self.char




def parse_rules(rules_raw):
    raw_dict = {}

    for rule in rules_raw:
        idx = int(rule.split(":")[0].strip())
        raw_dict[idx] = rule.split(":")[1].strip()

    parsed = {}

    return FinalRule(parse(raw_dict, parsed, 0))


def parse(raw_dict, parsed, rule_key):
    rule = raw_dict.pop(rule_key)


    if rule.startswith("\""):
        parsed[rule_key] = LeafRule(rule.replace("\"", ""))
    elif "|" not in rule:
        subrules = []

        for idx_s in rule.split(" "):
            idx = int(idx_s)

            if idx not in parsed:
                parse(raw_dict, parsed, idx)
            
            subrules.append(parsed[idx])

        parsed[rule_key] = SuperRule(subrules)
    else:
        sub_a, sub_b = rule.split("|")

        subs_a = []
        for idx_s in sub_a.strip().split(" "):
            idx = int(idx_s)
            
            if idx not in parsed:
                parse(raw_dict, parsed, idx)

            subs_a.append(parsed[idx])

        subs_b = []
        for idx_s in sub_b.strip().split(" "):
            idx = int(idx_s)
            
            if idx not in parsed:
                parse(raw_dict, parsed, idx)

            subs_b.append(parsed[idx])

        parsed[rule_key] = OptionalRule(SuperRule(subs_a), SuperRule(subs_b))
    
    return parsed[rule_key]
        


def main():
    with open(INPUT_FILE, "r") as infile:
        rules_text, messages_text = infile.read().strip().split("\n\n")


    rules_raw = list(map(lambda x: x.strip(), rules_text.split("\n")))

    top_rule = parse_rules(rules_raw)

    total = 0
    for line in messages_text.split("\n"):
        if top_rule.check_rule(list(line.strip())):
            total += 1

    print(top_rule)
    print(total)    

if __name__ == "__main__":
    main()