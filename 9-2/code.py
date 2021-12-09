import sys
import numpy as np

risk = np.genfromtxt(sys.argv[1],delimiter=1,dtype='int') + 1
much_risk = np.zeros((risk.shape[0] + 2, risk.shape[1] + 2))
much_risk.fill(sys.maxsize)
much_risk[1:risk.shape[0]+1,1:risk.shape[1]+1] = risk

for i in range(1,risk.shape[0]+1):
    for j in range(1,risk.shape[1]+1):
        if abs(much_risk[i-1,j]) > abs(much_risk[i,j]) and abs(much_risk[i+1,j]) > abs(much_risk[i,j]) and abs(much_risk[i,j-1]) > abs(much_risk[i,j]) and abs(much_risk[i,j+1]) > abs(much_risk[i,j]):
            much_risk[i,j] = -abs(much_risk[i,j])

s = np.where(much_risk < 0)

basin_depth = []

for i,j in zip(s[0], s[1]):
    c = abs(much_risk)
    c[c>10] = -abs(c[c>10])
    c[i,j]  = -abs(c[i,j])
    
    expanse = lambda x,y,z: [(a,b) for a,b in zip([x-1,x+1,x,x],[y,y,y-1,y+1]) if abs(z[x,y]) < abs(z[a,b]) and abs(z[a,b]) < 10 and z[a,b] > 0]
    traverse = [] + expanse(i,j,c)

    while len(traverse):
        ii,jj = traverse.pop()
        print(ii,jj)
        c[ii,jj] = -abs(c[ii,jj])
        traverse += expanse(ii,jj,c)

    t = c[1:risk.shape[0]+1,1:risk.shape[1]+1]
    if t[t<0].shape[0] == 113:
    basin_depth.append(t[t<0].shape[0])

print("Kowalsky, we need to go deeper!", np.prod(np.sort(basin_depth)[-3:]))