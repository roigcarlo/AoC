import re
import sys
import numpy as np

def solve_problem(generations):
    family = np.zeros(generations)

    for i in range(9):
        family[i] = i // 7

    for i in range(9, generations):
        family[i] = 1 + family[i-7] + family[i-9]
    
    return family

if __name__ == "__main__":
    g = int(sys.argv[2]) + 7
    pool = solve_problem(g)
    with open(sys.argv[1]) as data:
        pesesitos = np.array(re.findall('\d', data.read()), dtype='int')
        print("Kowalsky, fuck generative functions and their closed forms:", np.sum([pool[g-n-1] for n in pesesitos]) + len(pesesitos))