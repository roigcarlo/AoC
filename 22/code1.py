import re
import sys
import json
import time
import itertools

def remove_duplicated(a):
    return [(c[0], c[1], c[2], c[3], c[4], c[5], c[6]) for c in a]

def aabb_volumetric_intersection(a,b):
    return (((a[1] <= b[1] < a[2] or a[1] < b[2] <= a[2] or b[1] <= a[1] < b[2] or b[1] < a[2] <= b[2]) and 
             (a[3] <= b[3] < a[4] or a[3] < b[4] <= a[4] or b[3] <= a[3] < b[4] or b[3] < a[4] <= b[4]) and 
             (a[5] <= b[5] < a[6] or a[5] < b[6] <= a[6] or b[5] <= a[5] < b[6] or b[5] < a[6] <= b[6]) ))

def aabb_volumetric_isinside(b,a):
    return ( a[1] <= b[1] <= a[2] and a[1] <= b[2] <= a[2] and
             a[3] <= b[3] <= a[4] and a[3] <= b[4] <= a[4] and
             a[5] <= b[5] <= a[6] and a[5] <= b[6] <= a[6]
            ) 

def cubic_deconstruction(a,b):
    deconstructed_a = []
    deconstructed_b = []

    xes = sorted(list(set([a[1], a[2], b[1], b[2]])))
    yes = sorted(list(set([a[3], a[4], b[3], b[4]])))
    zes = sorted(list(set([a[5], a[6], b[5], b[6]])))

    px = [[xes[i], xes[i+1]] for i in range(len(xes)-1)]
    py = [[yes[i], yes[i+1]] for i in range(len(yes)-1)]
    pz = [[zes[i], zes[i+1]] for i in range(len(zes)-1)]

    exploded_cubes = [[a[0], *sum(c,[])] for c in itertools.product(px,py,pz)]

    for e_cube in exploded_cubes:
        
        ia = aabb_volumetric_isinside(e_cube, a)
        ib = aabb_volumetric_isinside(e_cube, b)

        if   ib: 
            e_cube[0] = b[0]
        elif ia:
            e_cube[0] = a[0]
        else:
            e_cube[0] = 'off'

        if ia:
            if e_cube[0] == 'on':
                deconstructed_a.append(e_cube)
    
    deconstructed_b.append(b)

    return deconstructed_a, deconstructed_b

ta = time.perf_counter()
with open(sys.argv[1]) as data:
    new_world = []
    old_world = []

    total_int = 0
    cubes = 0

    for l in data:
        mode, xl, xr, yl, yr, zl, zr = re.findall('(on|off|-?\d+)', l)
        new_cube = [mode, int(xl), int(xr)+1, int(yl), int(yr)+1, int(zl), int(zr)+1]
        
        old_challengers = [new_cube]
        new_challengers = []

        old_world = [c for c in old_world if c[0] == 'on' and not aabb_volumetric_isinside(c, new_cube)]

        while len(old_world):
            old_cube, old_world = old_world[0], old_world[1:]
            any_old_intersect = False
            while len(old_challengers):
                old_challenger, old_challengers = old_challengers[0], old_challengers[1:]
                if aabb_volumetric_intersection(old_cube, old_challenger):
                    any_old_intersect = True
                    new_cubes_a, new_cubes_b = cubic_deconstruction(old_cube, old_challenger)
                    new_world += new_cubes_a
                    new_challengers += new_cubes_b
            new_challengers = [new_cube]
            old_challengers, new_challengers = new_challengers, old_challengers

            if not any_old_intersect:
                new_world += [old_cube]
            else:
                new_world += old_challengers

        new_world = [c for c in new_world if c[0] == 'on' and not aabb_volumetric_isinside(c, new_cube)]
        if new_cube[0] == 'on':
            new_world.append(new_cube)

        old_world, new_world = new_world, old_world
        total_int +=len(old_world)
        print(len(old_world), total_int)

    actives = sum([(a[2]-a[1])*(a[4]-a[3])*(a[6]-a[5]) for a in old_world])
    
tb = time.perf_counter()
print(f"Kowalsky, go play minecraft: {actives}, {(tb-ta):0.2f}")

# Print for vox_render
if False:
    with open('E:/voxel.txt', 'w') as outfile:
        json.dump(yyy, outfile)
