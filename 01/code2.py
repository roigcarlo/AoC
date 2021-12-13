import sys

old = 0

inc = 0
ido = 0

w = [0, 0, 0]

with open(sys.argv[1]) as data:
	lines = [int(d.strip()) for d in data.readlines()]
	
	w[0] = lines[0] + lines[1]
	w[1] = lines[1]
	
	for i in range(2, len(lines)):
		w = [a+lines[i] for a in w]

		new = w[ido%3]
		w[ido%3] = 0
		ido += 1

		if new > old:
			inc += 1

		old = new

print(f"Turn all to the left!, we will go direction {inc-1}!")
