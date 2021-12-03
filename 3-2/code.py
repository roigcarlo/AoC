import sys
import numpy as np

def do(l,a,b,o):
    for i in range(o.shape[0]):
        if [o:=o[:,o[i,:]==l(o,a,b)[i]]] and o.shape[1]==1:
            return int(''.join([str(x) for x in o.T.reshape(o.shape[0],)]), 2)

o2,co2 = map(lambda a: do(*a),[(l:=lambda m,a,b: [(a,b)[np.count_nonzero(r) >= m.shape[1]/2] for r in m],0,1,m:=np.genfromtxt(sys.argv[1], delimiter=1, dtype='int').T), (l,1,0,m)])

print(f"Kowalsky, life support status!: {o2*co2}")