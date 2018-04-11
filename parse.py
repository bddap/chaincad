
def child(s):
    c = ""
    while s and not s[0].isspace() and s[0] != ']':
        c += s.pop(0)
    return c

def parse(s):
    while s:
        if s[0] == '[':
            s.pop(0)
            yield tuple(parse(s))
        elif s[0] == ']':
            s.pop(0)
            break
        elif s[0].isspace():
            s.pop(0)
        else:
            yield child(s)
