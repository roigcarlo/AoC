import re
import sys
import json
import time
import itertools

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

    return [cube for cube in exploded_cubes if aabb_volumetric_isinside(cube, a)]

def filter(a, new_cube):
    return [c for c in a if c[0] == 'on' and not aabb_volumetric_isinside(c, new_cube)]

def solve():
    with open(sys.argv[1]) as data:
        new_world = []
        old_world = []

        for l in data:
            mode, xl, xr, yl, yr, zl, zr = re.findall('(on|off|-?\d+)', l)
            new_cube = [mode, int(xl), int(xr)+1, int(yl), int(yr)+1, int(zl), int(zr)+1]

            for old_cube in old_world:
                new_world += cubic_deconstruction(old_cube, new_cube) if aabb_volumetric_intersection(old_cube, new_cube) else [old_cube]

            new_world = filter(new_world, new_cube)
            if new_cube[0] == 'on':
                new_world.append(new_cube)

            old_world, new_world = new_world, []

        actives = sum([(a[2]-a[1])*(a[4]-a[3])*(a[6]-a[5]) for a in old_world])
        return actives

ta = time.perf_counter()
actives = solve()
tb = time.perf_counter()
print(f"Kowalsky, go play minecraft: {actives}, {(tb-ta):0.2f}")

# Print for vox_render
if False:
    with open('E:/voxel.txt', 'w') as outfile:
        json.dump(yyy, outfile)
