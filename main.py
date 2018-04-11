#!/usr/bin/env python3

from parse import parse
from substitute import substitute
from compile import compile

if __name__ == '__main__':
    import sys
    inf = open(sys.argv[1]) if len(sys.argv) > 1 else sys.stdin
    # ouf = open(sys.argv[2]) if len(sys.argv) > 2 else sys.stdout
    ins = inf.read()
    clip = tuple(parse(list(ins)))
    print(clip)
    subs = substitute(clip)
    print(subs)
