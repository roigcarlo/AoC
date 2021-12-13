import sys
import numpy as np

m = np.genfromtxt(sys.argv[1], delimiter=1).T

g = int(''.join([('0','1')[np.count_nonzero(r) > m.shape[1]/2] for r in m]),2)
e = int(''.join([('1','0')[np.count_nonzero(r) > m.shape[1]/2] for r in m]),2)

print(f"Kowalsky, status report!: {g*e}")