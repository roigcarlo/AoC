import sys 
import numpy as np

np.set_printoptions(threshold=sys.maxsize)

m = np.genfromtxt(sys.argv[1], delimiter=1, dtype='int')
z = m[0,0]

l = m.shape[0] * 5
e = np.zeros(shape=(l, l), dtype='int')
s = np.zeros(shape=(l, l), dtype='int')

def calc_score(i,j):
    if i == l-1 and j == l-1:
        return e[i,j]

    if s[i,j] != 0:
        return s[i,j]

    if 

for i in range(0,5):
    for j in range(0,5):
        e[i*m.shape[0]:i*m.shape[0]+m.shape[0],j*m.shape[1]:j*m.shape[0]+m.shape[0]] = (m + i + j)

e[e>9] -= 9
c = e[0,0]

e[0,0] = 0
for i in range(1, e.shape[0]):
    e[i,0] += e[i-1,0]
    e[0,i] += e[0,i-1]

for i in range(1, e.shape[0]):
    for j in range(1, e.shape[1]):
        e[i,j] = e[i,j] + min(e[i-1,j], e[i,j-1])
np.savetxt('mc', e, fmt="%4d")

print("Kowalsky, mind the ceiling!:", e[-1,-1], c, z)