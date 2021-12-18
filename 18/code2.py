import re
import sys
import math
import time
import itertools

def tree_add(a):
    return 3 * tree_add(a[0]) + 2 * tree_add(a[1]) if isinstance(a, list) else a

def sub_l(n,l):
    l = l[::-1]
    if m:=re.search(r'\d\d*', l):
        chain = m.group(0) 
        rep = str(int(chain[::-1])+int(n))[::-1]
        return re.sub(r'\d\d*', rep, l, 1)[::-1]
    return l[::-1]

def sub_r(n,l):
    if m:=re.search(r'\d\d*', l):
        chain = m.group(0) 
        rep = str(int(chain)+int(n))
        return re.sub(r'\d\d*', rep, l, 1)
    return l

def explode(l):
    count = 0 
    for i in range(len(l)):
        if l[i] == '[': count += 1
        if l[i] == ']': count -= 1
        if count > 4:
            m = re.search(r'\d\d*,\d\d*', l[i:])
            l_n, r_n = m.group(0).split(',')
            l_l, r_l = l[:i], l[i+len(m.group(0))+2:]
            ls = sub_l(l_n, l_l)
            rs = sub_r(r_n, r_l)
            return True, f"{ls}0{rs}"
    return False, l

def split(l):
    if m:=re.search(r'\d\d+', l):
        div = int(m.group(0))/2
        rep = f"[{math.floor(div)},{math.ceil(div)}]"
        return True, re.sub(r'(\d\d+)', rep, l, 1)
    return False, l

def eval_number(n):
    k = True
    while k and [k := False]:
        if not k: k, n = explode(n)
        if not k: k, n = split(n)
    return tree_add(eval(n))

ta = time.perf_counter()
with open(sys.argv[1]) as data:
    max_eval = 0

    n = [l.strip() for l in data.readlines()]
    pairs = itertools.combinations(n,2)

    for p in pairs:
        max_eval = max(max_eval, eval_number(f"[{p[0]},{p[1]}]"))
        max_eval = max(max_eval, eval_number(f"[{p[1]},{p[0]}]"))
     
tb = time.perf_counter()
print(f"Kowalsky, that was.. hard?: {max_eval} {(tb-ta):0.2f}")
        
