import sys
import time

class Cavern:
    def __init__(self, name):
        self.name = name
        self.is_big = name.isupper()
        self.routes = []

class Daedalus:
    def __init__(self):
        self.caverns = {}
    
    def add_cavern(self, a, b):
        if a not in self.caverns: self.caverns[a] = Cavern(a)
        if b not in self.caverns: self.caverns[b] = Cavern(b)

        if b not in self.caverns[a].routes and b != 'start':
            self.caverns[a].routes.append(self.caverns[b])

        if a not in self.caverns[b].routes and a != 'start':
            self.caverns[b].routes.append(self.caverns[a])

def traverse(this_cavern, stack, did_backstep):
    if this_cavern.name == 'end':
        return 1

    this_level = 0
    for route in this_cavern.routes:
        sc = stack.count(route.name)
        if route.is_big or sc < 1 or not did_backstep and sc < 2:
            this_level += traverse(route, stack[:]+[route.name], did_backstep or not route.is_big and sc+1 > 1)

    return this_level

ta = time.perf_counter()
with open(sys.argv[1]) as data:

    labyrinth = Daedalus()

    for l in data:
        a,b = l.strip().split('-')
        labyrinth.add_cavern(a,b)

    paths = traverse(labyrinth.caverns['start'], ['start'], False)
tb = time.perf_counter()

print('Kowalsky, find the paths betwen the caves!:', paths, f"Solve: {(tb-ta):0.2f}")