import re
import sys
import time 

from collections import Counter

ta = time.perf_counter()
with open(sys.argv[1]) as data:
    chain = data.readline().strip()
    subs = {}
    steps = 10
    
    for l in data:
        injection = re.search(r'(\w\w) -> (\w)', l)
        if injection:
            src = injection.group(1)
            dst = injection.group(2)
            subs[injection.group(1)] = src[0] + dst

    def encode(match): 
        lookahead_match = ''.join(match.groups())
        return subs[lookahead_match] if lookahead_match in subs else lookahead_match
 
    for i in range(steps):
        chain = re.sub(r'(\w)(?=(\w))', encode, chain)

    chain_counter = Counter(chain)

tb = time.perf_counter()
print(f"Kowalsky, believe in the heart of the cards!: {len(chain)}, {max(chain_counter.values()) - min(chain_counter.values())}, {1000*(tb-ta):0.2f}")

    