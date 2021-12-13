import sys

langs = {'(':')','[':']' ,'{':'}' ,'<':'>'}
score = {'(':1  ,'[':2   ,'{':3   ,'<':4  }

def perl(l):
    stack = []
    for oc in l:
        if oc in langs.keys():
            stack += oc
        if oc in langs.values():
            cc = stack.pop()
            if oc != langs[cc]:
                return 0

    autocomple_score = 0
    for c in stack[::-1]:
        autocomple_score *= 5
        autocomple_score += score[c] 

    return autocomple_score

with open(sys.argv[1]) as data:
    res = list(filter(lambda x: x > 0, [perl(l) for l in data]))
    res.sort()
        
    print("Kowalsky, what a shame not to use recursive regex to solve this!:", res[len(res)//2])
            