arr = {'sweet', 'home', 'alabama'}

def compose(formed, s):
    if len(s) == 0:
        print(formed[0].upper() + str(formed[1:]))
    for e in s:
        compose(formed + e + " ", s - {e})

compose("", arr)
