import sys
import numpy as np

m = np.genfromtxt(sys.argv[1], delimiter=1, dtype='int').T

g,o = [(0,1)[np.count_nonzero(r) > m.shape[1]/2] for r in m],m
e,c = [(1,0)[np.count_nonzero(r) > m.shape[1]/2] for r in m],m

for i in range(len(g)):
    o = o[:, o[i,:]==g[i]]
    g = [(0,1)[np.count_nonzero(r) >= o.shape[1]/2] for r in o]
    if o.shape[1] == 1:
        o2  = int(''.join([str(x) for x in o.T.reshape(o.shape[0],)]), 2)

for i in range(len(g)):
    c = c[:, c[i,:]==e[i]]
    e = [(1,0)[np.count_nonzero(r) >= c.shape[1]/2] for r in c]
    if c.shape[1] == 1:
        co2 = int(''.join([str(x) for x in c.T.reshape(c.shape[0],)]), 2)

print(f"Kowalsky, life support status!: {o2*co2}")