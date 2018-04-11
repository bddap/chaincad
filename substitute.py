
def substitute(parsed):
    if type(parsed) is str:
        return parsed
    if type(parsed) is tuple:
        return tuple(substitute(p) for p in parsed)
    
