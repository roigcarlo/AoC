import sys 
import numpy as np

m = np.genfromtxt(sys.argv[1], delimiter=1)
c = m[0,0]

for i in range(1, m.shape[0]):
    m[i,0] += m[i-1,0]
    m[0,i] += m[0,i-1]

for i in range(1, m.shape[0]):
    for j in range(1, m.shape[1]):
        m[i,j] = m[i,j] + min(m[i-1,j], m[i,j-1])

print(m)

print("Kowalsky, mind the ceiling!:", m[-1,-1]-c)