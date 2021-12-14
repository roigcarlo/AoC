import re
import sys
import time

ta = time.perf_counter()
with open(sys.argv[1]) as data:
    chain = data.readline().strip()
    s,z = {},{}
    steps = 40
    
    for l in data:
        injection = re.search(r'(\w\w) -> (\w)', l)
        if injection:
            s[injection.group(1)] = injection.group(2)
            z[injection.group(2)] = 0

    m = {k:[k[0]+s[k], s[k]+k[1], 0] for k in s}
    n = {k:[k[0]+s[k], s[k]+k[1], 0] for k in s}

    for p in re.findall(r'(?=(\w{2}))', chain):
        m[p][2] += 1

    for i in range(steps):
        for p in m:
            a,b,c = m[p]
            n[a][2] += c
            n[b][2] += c
            m[p][2] -= c 
        m,n = n,m

    for p in m:
        z[p[0]] += m[p][2]
    z[chain[-1]] += 1
tb = time.perf_counter()

print(f"Kowalsky, believe in the heart of the cards!: {max(z.values()) - min(z.values())} {1000*(tb-ta):0.2f}")