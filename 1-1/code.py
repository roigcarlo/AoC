import sys

old = -1
inc =  0

with open(sys.argv[1]) as data:
	for l in data.readlines():
		new = int(l.strip())
		if new > old:
			inc += 1
		old = new

print(f"Admiral Kowalsky, the route {inc-1} is clear!")
