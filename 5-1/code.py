import re
import sys
import numpy as np

with open(sys.argv[1]) as data:

    vents = np.array(re.findall('(\d+),(\d+) -> (\d+),(\d+)', data.read()), dtype='int')
    field = np.zeros(shape=(np.max(vents) + 1, np.max(vents) + 1))

    for v in vents:
        if v[0] == v[2] or  v[1] == v[3]:
            field[min(v[0],v[2]):max(v[0],v[2])+1,min(v[1],v[3]):max(v[1],v[3])+1] += 1
    
    print("Kowalsky, report geothermal activity!:", np.count_nonzero(field[field>1]))

