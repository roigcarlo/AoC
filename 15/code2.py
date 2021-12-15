import sys 
import time
import numpy as np

ta = time.perf_counter()
m = np.genfromtxt(sys.argv[1], delimiter=1, dtype='int')
l  = m.shape[0] * 5
e = np.zeros(shape=(l, l), dtype='int')

for i in range(0,5):
    for j in range(0,5):
        e[i*m.shape[0]:i*m.shape[0]+m.shape[0],j*m.shape[1]:j*m.shape[0]+m.shape[0]] = (m + i + j)

e[e>9]-=9
e[0,0] = 0
l = e.shape[0] 
s = np.zeros(shape=(l, l), dtype='int')
s.fill(l*l*10)
s[0,0] = 0 

def multidimensional_traverse():
    cells = {e[0,0]:[(0,0)]}
    w = e[0, 0]
    while len(cells):
        level = cells.pop(w, None)
        if level:
            for c in level:
                if c[0] == l-1 and c[1] == l-1:
                    return w
                v = s[c[0],c[1]]
                if c[0] < l-1 and e[c[0]+1,c[1]]+v < s[c[0]+1,c[1]]:
                    s[c[0]+1,c[1]] = e[c[0]+1,c[1]]+v
                    idx = abs(s[c[0]+1,c[1]])
                    if idx not in cells.keys():
                        cells[idx] = []
                    cells[idx].append((c[0]+1,c[1]))
                if c[0] > 0 and e[c[0]-1,c[1]]+v < s[c[0]-1,c[1]]:
                    s[c[0]-1,c[1]] = e[c[0]-1,c[1]]+v
                    idx = abs(s[c[0]-1,c[1]])
                    if idx not in cells.keys():
                        cells[idx] = []
                    cells[idx].append((c[0]-1,c[1]))
                if c[1] < l-1 and e[c[0],c[1]+1]+v < s[c[0],c[1]+1]:
                    s[c[0],c[1]+1] = e[c[0],c[1]+1]+v
                    idx = abs(s[c[0],c[1]+1])
                    if idx not in cells.keys():
                        cells[idx] = []
                    cells[idx].append((c[0],c[1]+1))
                if c[1] > 0 and e[c[0],c[1]-1]+v < s[c[0],c[1]-1]:
                    s[c[0],c[1]-1] = e[c[0],c[1]-1]+v
                    idx = abs(s[c[0],c[1]-1])
                    if idx not in cells.keys():
                        cells[idx] = []
                    cells[idx].append((c[0],c[1]-1))
        w += 1

result = multidimensional_traverse()
tb = time.perf_counter()
print("Kowalsky, IT LIVES, IT LIVESSSSSS HAHAHAHAHAHAHA!:", result, (tb-ta))