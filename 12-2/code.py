import sys

class Cavern:
    def __init__(self, name):
        self.name = name
        self.is_big = name.isupper()
        self.routes = []

    def __repr__(self):
        return f"{self.name},{self.is_big}"

class Daedalus:
    def __init__(self):
        self.caverns = {}
    
    def add_cavern(self, a, b):
        if a not in self.caverns:
            self.caverns[a] = Cavern(a)

        if b not in self.caverns:
            self.caverns[b] = Cavern(b)

        if b not in self.caverns[a].routes:
            self.caverns[a].routes.append(self.caverns[b])

        if a not in self.caverns[b].routes:
            self.caverns[b].routes.append(self.caverns[a])

def traverse(this_cavern, stack, did_backstep):
    if this_cavern.name == 'end':
        return [stack]

    this_level = []
    for route in this_cavern.routes:
        if route.is_big or (
            route.name != 'start' and 
            not route.is_big and ( 
                (stack.count(route.name) <= 1 and not did_backstep) or 
                (stack.count(route.name) <= 0 ) 
            )
        ):
            temp_stack = stack[:]
            temp_stack.append(route.name)
            new_paths = traverse(route, temp_stack, did_backstep or (not route.is_big and temp_stack.count(route.name) > 1))

            for p in new_paths:
                if p[-1] == 'end':
                    this_level.append(p)

    return this_level

with open(sys.argv[1]) as data:

    labyrinth = Daedalus()

    for l in data:
        a,b = l.strip().split('-')
        labyrinth.add_cavern(a,b)

    paths = traverse(labyrinth.caverns['start'], ['start'], False)

    print('Kowalsky, find the paths betwen the caves!:', len(paths))
    