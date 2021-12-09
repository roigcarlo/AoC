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

s = sum(much_risk[much_risk < 0])
print("Kowalsky, down down town!", -s)