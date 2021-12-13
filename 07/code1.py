import re
import sys
import numpy as np

if __name__ == "__main__":
    with open(sys.argv[1]) as data:
        crabs = np.array(re.findall('\d+', data.read()), dtype='int')
        max_crabs = np.max(crabs)
        min_fuel  = sys.maxsize

        flotilla = np.zeros(max_crabs+1)
        fuel_l, crab_l = 0,0
        fuel_c, crab_c = 0,0
        fuel_r, crab_r = 0,len(crabs)

        for crab in crabs:
           flotilla[crab] += 1
           fuel_r += crab

        fuel_c  = flotilla[0]
        crab_c  = flotilla[0]
        crab_r -= flotilla[0]

        for i in range(max_crabs):
            if fuel_l + fuel_r < min_fuel:
                min_fuel = fuel_l + fuel_r

            crab_l += crab_c
            crab_c  = flotilla[i+1]
            crab_r -= crab_c
            fuel_l += crab_l
            fuel_c  = flotilla[i+1]
            fuel_r -= crab_r + crab_c
        
        print("Kowalsky this is better!:", min_fuel)