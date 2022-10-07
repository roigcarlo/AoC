import re
import sys

print("Kowlasky:", sum([len([n for n in re.findall('(\w+)\s', l.split('|')[1]) if len(n) in [2,3,4,7]]) for l in open(sys.argv[1]).readlines()]))