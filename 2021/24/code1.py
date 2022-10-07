import sys
import time
import functools

import cProfile, pstats, io
from pstats import SortKey
pr = cProfile.Profile()

def clang(code, size):
    for i in range(0, len(code), size):
        yield code[i:i+size]

def python(program, memory, input):
    for line in program:
        inst = line.split(' ')

        if inst[0] == 'inp':
            memory[inst[1]] = input
        if inst[0] == 'add':
            memory[inst[1]] +=  memory[inst[2]] if inst[2] in memory.keys() else int(inst[2])
        if inst[0] == 'mul':
            memory[inst[1]] *=  memory[inst[2]] if inst[2] in memory.keys() else int(inst[2])
        if inst[0] == 'div':
            memory[inst[1]] //= memory[inst[2]] if inst[2] in memory.keys() else int(inst[2])
        if inst[0] == 'mod':
            memory[inst[1]] %=  memory[inst[2]] if inst[2] in memory.keys() else int(inst[2])
        if inst[0] == 'eql':
            memory[inst[1]] =   int(memory[inst[1]] == (memory[inst[2]] if inst[2] in memory.keys() else int(inst[2])))
    
    return memory

@functools.cache
def decompiled(a,b,c,d,e,f,z,w):
    X = w!=((z%a)+c)
    Y = z//b
    Z = Y*((X*d)+e)+(X*(w+f))
    return Z

def read_program(program):
    a = int(program[3].split(' ')[2])
    b = int(program[4].split(' ')[2])
    c = int(program[5].split(' ')[2])
    d = int(program[9].split(' ')[2])
    e = int(program[11].split(' ')[2])
    f = int(program[15].split(' ')[2])
    return a,b,c,d,e,f

@functools.cache
def find_digit(program,target,code_id):
    if code_id == 14:
        return '0'

    code = program[code_id]

    a,b,c,d,e,f = read_program(code)

    candidates = []

    for n in range(1,10):
        # If our vision window is open, we look into what C is marking
        if b == 26:
            # Look the waste zone
            guess = n-c
            if target % 26 == f+n:
                objective = target // 26
                guess = 26 * objective
                for i in range(guess,guess+26):
                    residual = decompiled(a,b,c,d,e,f,i,n)
                    if residual == target:
                        candidates.append((str(n), i))

            # Look the residuals
            guess = n-c
            residual = decompiled(a,b,c,d,e,f,guess,n)

            # First look for the natural residuals:
            # If greater than our target, look down, esle look up
            if   residual < target:
                while residual < target:
                    guess += 26
                    residual = decompiled(a,b,c,d,e,f,guess,n)
            elif residual > target:
                while residual > target:
                    guess -= 26
                    residual = decompiled(a,b,c,d,e,f,guess,n)

            if residual == target:
                candidates.append((str(n), guess))
        else:
            # We don't have trash to explore, so get at the initial pointer and scroll
            guess = n-c
            residual = decompiled(a,b,c,d,e,f,guess,n)

            # First look for the natural residuals:
            # If greater than our target, look down, esle look up
            if   residual < target:
                while residual < target:
                    guess += 1
                    residual = decompiled(a,b,c,d,e,f,guess,n)
            elif residual > target:
                while residual > target:
                    guess -= 1
                    residual = decompiled(a,b,c,d,e,f,guess,n)

            if residual == target:
                candidates.append((str(n), guess))

    max_num = 0
    for pair in sorted(candidates, reverse=True):
        max_num = max(int(find_digit(program,pair[1],code_id+1) + str(pair[0])), max_num)

    return str(max_num)

ta = time.perf_counter()
with open(sys.argv[1]) as data:
    memory = {'w':0,'x':0,'y':0,'z':0}

    code = [l.strip() for l in data.readlines()]
    code = list(clang(code,18))
    code = code[::-1]

    code = tuple([tuple(cl for cl in l) for l in code])

    serial_number = find_digit(code,0,0)
tb = time.perf_counter()

print(f"Kowlasky, prepare for reaching the trench bottom!: {serial_number} {(tb-ta):0.2f}")