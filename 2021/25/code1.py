import sys
import numpy as np

w = ['>','v']
m = np.genfromtxt(sys.argv[1], delimiter=1, dtype='str')

j,s = 0,0
while not s:
    n = np.copy(m)
    for i in range(2):
        l = np.logical_and(np.roll(m,1,(i+1)%2)==[w[i%2]],m=='.')
        p = np.roll(l,-1,(i+1)%2)
        m[l] = w[i%2]
        m[p] = '.'
    j+=1
    s = np.array_equal(m,n)
print(f"Kowalsky, get the God damn Keys already!: {j}")