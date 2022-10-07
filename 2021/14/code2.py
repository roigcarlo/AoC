import sys
import time

ta = time.perf_counter()
with open(sys.argv[1]) as data:
    chain = data.readline().strip()
    s,z = {},{}
    
    s = {l[0:2]:l[6] for l in data.readlines()[1:]}
    m = {k:[k[0]+s[k], s[k]+k[1], 0] for k in s}
    n = {k:[k[0]+s[k], s[k]+k[1], 0] for k in s}

    for p in [chain[i:i+2] for i in range(len(chain)-1)]:
        m[p][2] += 1

    j,k = 2,3
    for i in range(40):
        for p in m:
            a,b,c = m[p]
            if c: n[a][2] += c
            if c: n[b][2] += c
            m[p][2] -= c 
        m,n = n,m

    for p in m:
        z[p[0]] = 0

    for p in m:
        z[p[0]] += m[p][2]
    z[chain[-1]] += 1

tb = time.perf_counter()

print(f"Kowalsky, believe in the heart of the cards!: {max(z.values()) - min(z.values())} {1000*(tb-ta):0.2f}")