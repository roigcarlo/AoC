import re
import sys

with open(sys.argv[1]) as data:
    max_x = -1
    max_y = -1

    black_holes = set()
    for l in data:
        if ',' in l:
            x,y = map(int, l.strip().split(','))
            if x > max_x : max_x = x
            if y > max_y : max_y = y
            black_holes.add((x,y))

        if 'fold' in l:
            m = re.search('(\w+)=(\d+)', l)

            fold_space_and_time = m.group(1)
            warp_speed = int(m.group(2))

            if fold_space_and_time == 'x':
                black_holes = {((bh[0],max_x-bh[0])[bh[0]>warp_speed], bh[1]) for bh in black_holes}
                max_x /= 2

            if fold_space_and_time == 'y':
                black_holes = {(bh[0], (bh[1],max_y-bh[1])[bh[1]>warp_speed]) for bh in black_holes}
                max_y /= 2

            print("Kowalsky, engage the warp drive!:", len(black_holes))
            
            # Shamesly exit here.
            exit(0)