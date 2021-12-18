import sys
import re
import math


def tree_add(a):
    if isinstance(a, list):
        return 3 * tree_add(a[0]) + 2 * tree_add(a[1])
    else:
        return a 

def sub_l(n,l):
    l = l[::-1]
    m = re.search('\d\d*', l)
    if m:
        chain = m.group(0) 
        rep = str(int(chain[::-1])+int(n))[::-1]
        return re.sub('(\d\d*)', rep, l, 1)[::-1]
    return l[::-1]

def sub_r(n,l):
    m = re.search('\d\d*', l)
    if m:
        chain = m.group(0) 
        rep = str(int(chain)+int(n))
        return re.sub('(\d\d*)', rep, l, 1)
    return l

def explode(l):
    count = 0 
    for i in range(len(l)):
        if l[i] == '[': count += 1
        if l[i] == ']': count -= 1
        if count > 4:
            break
    if count > 4:
        m = re.search('\d\d*,\d\d*', l[i:])
        l_n, r_n = m.group(0).split(',')
        l_l, r_l = l[:i], l[i+len(m.group(0))+2:]
        ls = sub_l(l_n, l_l)
        rs = sub_r(r_n, r_l)
        return True, ls + str(0) + rs
    return False, l

def split(l):
    m = re.search('\d\d+', l)
    if m:
        rep = f"[{math.floor(int(m.group(0))/2)},{math.ceil(int(m.group(0))/2)}]"
        return True, re.sub('(\d\d+)', rep, l, 1)
    return False, l

with open(sys.argv[1]) as data:

    n = data.readline().strip()
    for l in data:
        n = f"[{n},{l.strip()}]"
        k = True
        while k:
            k = False
            if not k: 
                k, n = explode(n)
            if not k: 
                k, n = split(n)

    tree = tree_add(eval(n))
     
    print("Kowalsky, that was.. hard?:", tree)
        