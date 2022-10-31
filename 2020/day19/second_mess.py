from abc import ABC
from copy import deepcopy
from typing import Optional

INPUT_FILE = "./test.txt"

class BaseRule(ABC):
    def check_rule(self, text_stack):
        pass

    def __str__(self):
        if self.idx is not None:
            return self.__class__.__name__ + ":" + str(self.idx)


class FinalRule(BaseRule):
    def __init__(self, subrule, idx=None):
        self.subrule = subrule
        self.idx = idx

    def check_rule(self, text_stack):
        success = self.subrule.check_rule(text_stack)

        return success and len(text_stack) == 0


class SuperRule(BaseRule):
    def __init__(self, subrules, idx=None):
        self.subrules = subrules
        self.idx = idx

    def check_rule(self, text_stack):
        print(self.idx)

        if self.idx == 15:
            print(list(map(lambda x: str(x), self.subrules)))

        i = 0
        for subrule in self.subrules:
            print(f"{self.idx} seq: check number {i+1}/{len(self.subrules)}")
            i += 1
            if not subrule.check_rule(text_stack):
                return False
        
        print(f"{self.idx} seq: success.")
        return True

    def add_rule(self, rule):
        self.subrules.append(rule)



class OptionalRule(BaseRule):
    subrule_a: BaseRule
    subrule_b: BaseRule

    def __init__(self, subrule_a, subrule_b, idx=None):
        self.subrule_a = subrule_a
        self.subrule_b = subrule_b
        self.idx = idx

    def check_rule(self, text_stack):
        # print(str(self.idx) + " - opt")
        stack_clone = deepcopy(text_stack)

        if (self.idx == 8 or self.idx == 11) and 1 == 2:
            sa = self.subrule_a.subrules
            sb = self.subrule_b.subrules


            print(f"{self.idx} = A: ", end="")
            for x in sa:
                print(x, end=", ")

            print(" B: ", end="")
            for x in sb:
                print(x, end=", ")
            print()

        print(f"{self.idx}: Branch a with " + "".join(stack_clone))
        if self.subrule_a.check_rule(stack_clone):

            text_stack.clear()
            for element in stack_clone:
                text_stack.append(element)


            return True
        else:
            print(f"{self.idx}: Branch b with " + "".join(stack_clone))
            b_success = self.subrule_b.check_rule(text_stack)

            return b_success

    def add_b(self, rule):
        self.subrule_b.add_rule(rule)




class LeafRule(BaseRule):
    def __init__(self, char, idx=None):
        self.char = char
        self.idx = idx

    def check_rule(self, text_stack):
        print(f"{self.idx} - {self.char}")
        # print("Checking", self.char, "against", "".join(text_stack))

        success = len(text_stack) > 0 and text_stack.pop(0) == self.char
        
        if not success:
            print("Failed.")
            print()

        return success
        # return len(text_stack) > 0 and text_stack.pop(0) == self.char




def parse_rules(rules_raw):
    raw_dict = {}

    for rule in rules_raw:
        idx = int(rule.split(":")[0].strip())
        raw_dict[idx] = rule.split(":")[1].strip()

    # raw_dict[8] = "42 | 42 8"
    # raw_dict[11] = "42 31 | 42 11 31"

    parsed = {}

    top = FinalRule(parse(raw_dict, parsed, 0))

    # eight = OptionalRule(SuperRule([parsed[8]]), SuperRule([parsed[42], eight]))

    parsed[0] = LeafRule("a")

    return top


def parse(raw_dict, parsed, rule_key):
    rule = raw_dict.pop(rule_key)

    if rule_key == 8:
        if 42 not in parsed:
            parse(raw_dict, parsed, 42)

        left = SuperRule([parsed[42]], 8)
        right = SuperRule([parsed[42]], 8)

        eight = OptionalRule(left, right, 8)
        eight.add_b(eight)

        # print(list(map(lambda x: str(x), eight.subrule_b.subrules)))

        parsed[8] = eight

    elif rule_key == 11:
        if 42 not in parsed:
            parse(raw_dict, parsed, 42)

        if 31 not in parsed:
            parse(raw_dict, parsed, 31)

        left = SuperRule([parsed[42], parsed[31]], 11)
        right = SuperRule([parsed[42]], 11)

        eleven = OptionalRule(left, right, 11)
        eleven.add_b(eleven)
        eleven.add_b(parsed[31])

        parsed[11] = eleven


    elif rule.startswith("\""):
        parsed[rule_key] = LeafRule(rule.replace("\"", ""), rule_key)
    elif "|" not in rule:
        subrules = []

        for idx_s in rule.split(" "):
            idx = int(idx_s)

            if idx not in parsed:
                parse(raw_dict, parsed, idx)
            
            subrules.append(parsed[idx])

        parsed[rule_key] = SuperRule(subrules, rule_key)
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

        parsed[rule_key] = OptionalRule(SuperRule(subs_a, rule_key), SuperRule(subs_b, rule_key), rule_key)
    
    return parsed[rule_key]
        

def debug():
    # expr = "babbbbaabbbbbabbbbbbaabaaabaaa"
    expr = "aaaaabbaabaaaaababaa"

    with open(INPUT_FILE, "r") as infile:
        rules_text, messages_text = infile.read().strip().split("\n\n")

    rules_raw = list(map(lambda x: x.strip(), rules_text.split("\n")))

    top_rule = parse_rules(rules_raw)

    res = top_rule.check_rule(list(expr))

    print(res)


def main(rr):
    with open(INPUT_FILE, "r") as infile:
        rules_text, messages_text = infile.read().strip().split("\n\n")


    rules_raw = list(map(lambda x: x.strip(), rules_text.split("\n")))

    top_rule = parse_rules(rules_raw)

    total = 0
    for line in messages_text.split("\n"):
        ll = line.strip()

        mine = top_rule.check_rule(list(line))

        his = bool(rr.match(ll))

        if mine != his:
            print(line, mine, his)


        if top_rule.check_rule(list(line.strip())):
            total += 1

    print(total)    




# import re

# datafile = 'puzzle.txt'

# with open(datafile) as fh:
#     txt = fh.read()
#     rulestxt, datatxt = txt.split('\n\n')

# data = [y for y in (x.strip() for x in datatxt.split('\n')) if y]

# def make_rules(lines):
#     D = {}
#     for line in lines:
#         if not line:
#             continue
#         k, v = line.strip().split(':')
#         v = v.replace('"', '')
#         if '|' in v:
#             v = '(?: ' + v + ' )'
#         D[k] = v.split()
#     return D

# rules = make_rules(rulestxt.split('\n'))

# def rules_to_re(rules):
#     L = rules['0'].copy()
#     while any(x.isdigit() for x in L):
#         i, k = next((i,x) for (i, x) in enumerate(L) if x.isdigit())
#         L[i:i+1] = rules[k].copy()
#     L.insert(0, '^')
#     L.append('$')
#     return re.compile(''.join(L))

# rules_2 = make_rules(rulestxt.split('\n'))
# rules_2['8'] = ['(?:', '42', ')+']
# rules_2['11'] = [
#     '(?:',
#     '(?:', '(?:', '42', ')', '{1}', '(?:', '31', ')', '{1}', ')', '|',
#     '(?:', '(?:', '42', ')', '{2}', '(?:', '31', ')', '{2}', ')', '|',
#     '(?:', '(?:', '42', ')', '{3}', '(?:', '31', ')', '{3}', ')', '|',
#     '(?:', '(?:', '42', ')', '{4}', '(?:', '31', ')', '{4}', ')', '|',
#     '(?:', '(?:', '42', ')', '{5}', '(?:', '31', ')', '{5}', ')', '|',
#     '(?:', '(?:', '42', ')', '{6}', '(?:', '31', ')', '{6}', ')', '|',
#     '(?:', '(?:', '42', ')', '{7}', '(?:', '31', ')', '{7}', ')', '|',
#     '(?:', '(?:', '42', ')', '{8}', '(?:', '31', ')', '{8}', ')', '|',
#     '(?:', '(?:', '42', ')', '{9}', '(?:', '31', ')', '{9}', ')',
#     ')'
# ]

# rules_re_2 = rules_to_re(rules_2)

# print(rules_re_2)

if __name__ == "__main__":
    debug()

# part_2 = sum(bool(rules_re_2.match(x)) for x in data)
