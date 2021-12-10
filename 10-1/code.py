import sys

langs = {'(':')','[':']' ,'{':'}' ,'<':'>'}
score = {')':3  ,']':57  ,'}':1197,'>':25137}

def perl(l):
    stack = []
    for oc in l:
        if oc in langs.keys():
            stack += oc
        if oc in langs.values():
            cc = stack.pop()
            if oc != langs[cc]:
                return score[oc]
    return 0

with open(sys.argv[1]) as data:
    print("Kowalsky, what a shame not to use recursive regex to solve this!:", sum([perl(l) for l in data]))
            