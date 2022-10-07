import sys
import numpy as np

def do(e,l,f,s):return [w[0] for w in s[0:]] if not [t:=e(s,l,f)] or not[s:=s[:,s[0,:]==t[0]]] or s.shape[1]==1 else [t[0]] + do(e,l,f,s[1:])
o,c = map(lambda l:int(''.join([str(w)for w in do(*l)]),2),[(e:=lambda m,l,f:[(l,f)[np.count_nonzero(r)>=m.shape[1]/2] for r in m],0,1,m:=np.genfromtxt(sys.argv[1],delimiter=1,dtype='int').T),(e,1,0,m)])

print(f"Kowalsky!: {o*c}")