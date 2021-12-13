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
    epochs = int(sys.argv[2]) if len(sys.argv) == 3 else 250
    epochs += 7
    pool = solve_problem(epochs)
    with open(sys.argv[1]) as data:
        pesesitos = np.array(re.findall('\d', data.read()), dtype='int')
        print("Kowalsky, fuck generative functions and their closed forms:", np.sum([pool[epochs-n-1] for n in pesesitos]) + len(pesesitos))