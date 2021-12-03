import sys
import numpy as np

def do(e,l,f,s):
    for i in range(s.shape[0]):
        if [s:=s[:,s[i,:]==e(s,l,f)[i]]] and s.shape[1]==1:return int(''.join([str(x) for x in s.T.reshape(s.shape[0],)]),2)

o,c = map(lambda l:do(*l),[(e:=lambda m,l,f:[(l,f)[np.count_nonzero(r)>=m.shape[1]/2] for r in m],0,1,m:=np.genfromtxt(sys.argv[1],delimiter=1,dtype='int').T),(e,1,0,m)])

print(f"Kowalsky!: {o*c}")