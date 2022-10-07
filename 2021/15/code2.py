import sys 
import time
import numpy as np

def multidimensional_traverse(e,s,l):
    cells, w = {e[0,0]:[(0,0)]}, e[0, 0]
    while len(cells):
        level = cells.pop(w, None)
        for c in level:
            if c[0] == l-1 and c[1] == l-1: return w
            v = s[c[0],c[1]]
            R = c[0]+1,c[1]; L = c[0]-1,c[1]; B = c[0],c[1]+1; T = c[0],c[1]-1
            if c[0] < l-1 and e[R]+v < s[R]:
                s[R] = e[R]+v
                if s[R] not in cells.keys(): cells[s[R]] = []
                cells[s[R]].append((R))
            if c[0] > 0   and e[L]+v < s[L]:
                s[L] = e[L]+v
                if s[L] not in cells.keys(): cells[s[L]] = []
                cells[s[L]].append((L))
            if c[1] < l-1 and e[B]+v < s[B]:
                s[B] = e[B]+v
                if s[B] not in cells.keys(): cells[s[B]] = []
                cells[s[B]].append((B))
            if c[1] > 0   and e[T]+v < s[T]:
                s[T] = e[T]+v
                if s[T] not in cells.keys(): cells[s[T]] = []
                cells[s[T]].append((T))
        w = min(cells.keys())

m = np.genfromtxt(sys.argv[1], delimiter=1, dtype='int')
l = m.shape[0] * 5
e = np.zeros(shape=(l, l), dtype='int')

for i in range(0,5):
    for j in range(0,5):
        e[i*m.shape[0]:i*m.shape[0]+m.shape[0],j*m.shape[1]:j*m.shape[0]+m.shape[0]] = (m + i + j)

l = e.shape[0] 
s = np.zeros(shape=(l, l), dtype='int')
s.fill(l*l*10)

e[e>9]-= 9
e[0,0] = 0
s[0,0] = 0 

ta = time.perf_counter()
result = multidimensional_traverse(e,s,l)
tb = time.perf_counter()

print("Kowalsky, IT LIVES, IT LIVESSSSSS HAHAHAHAHAHAHA!:", result, (tb-ta))