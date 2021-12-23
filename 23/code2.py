import sys
import time
import functools

cost   = {'A':1,'B':10,'C':100,'D':1000}
target = {2:'A',3:'B',4:'C',5:'D'}
dest   = {'A':2,'B':3,'C':4,'D':5}
valid  = [1,1,0,1,0,1,0,1,0,1,1]
align  = {2:2,3:4,4:6,5:8}

global_min = {'value': sys.maxsize, 'recursions': 0}

def pretty_print(stat):
    print("#"*13)
    print("#"+''.join(stat[1])+"#")
    print("###"+stat[2][0]+"#"+stat[3][0]+"#"+stat[4][0]+"#"+stat[5][0]+"###")
    print("  #"+stat[2][1]+"#"+stat[3][1]+"#"+stat[4][1]+"#"+stat[5][1]+"#  ")
    print("  #"+stat[2][2]+"#"+stat[3][2]+"#"+stat[4][2]+"#"+stat[5][2]+"#  ")
    print("  #"+stat[2][3]+"#"+stat[3][3]+"#"+stat[4][3]+"#"+stat[5][3]+"#  ")
    print("  "+"#"*9)
    print(f"  <<{stat[0]:5d}>> ({global_min['value']})")

def can_push(room, rid):
    index = -1
    for i in range(len(room)):
        if room[i] == '.': index = i
        if room[i] != '.' and room[i] != target[rid]: index = -1
    return index

def can_pop(room, rid):
    index = -1
    for i in range(len(room)):
        if room[i] != '.' and room[i] != target[rid]: return i
        if room[i] != '.' and room[i] == target[rid]: return -1 if all(e == room[i] for e in room[i:]) else i
    return index

@functools.cache
def find_true_void(stat):
    if stat[1] == ('.','.','.','.','.','.','.','.','.','.','.') and stat[2] ==('A', 'A', 'A', 'A') and stat[3] == ('B', 'B', 'B', 'B') and stat[4] == ('C', 'C', 'C', 'C') and stat[5] == ('D', 'D', 'D', 'D'):
        return stat[0]

    stat_queue = {}

    # Move the ampis from the hallway
    found_correct_pos = False
    for i in range(11):
        if stat[1][i] != '.':
            room_dst = dest[stat[1][i]]
            
            pos_a, pos_b = i, align[room_dst]
            if pos_a > pos_b: pos_a, pos_b = pos_b, pos_a
            else: pos_a, pos_b = pos_a +1, pos_b + 1

            if stat[1][pos_a:pos_b] == tuple(['.' for _ in range(abs(i-align[room_dst]))]):
                push_depth = can_push(stat[room_dst], room_dst)
                if push_depth != -1:
                    new_stat = [stat[0], [a for a in stat[1]], [a for a in stat[2]], [a for a in stat[3]], [a for a in stat[4]], [a for a in stat[5]]]
                    new_stat[0] += cost[stat[1][i]] * (1+push_depth+abs(align[room_dst]-i))
                    new_stat[room_dst][push_depth] = stat[1][i]
                    new_stat[1][i] = '.'

                    if new_stat[0] not in stat_queue.keys(): stat_queue[new_stat[0]] = []
                    stat_queue[new_stat[0]].append((new_stat[0],)+tuple([tuple([a for a in b]) for b in new_stat[1:]]))
                    found_correct_pos = True
                    break

    if not found_correct_pos:
    # Move ampis out of the rooms
        for i in range(5,1,-1):
            pop_depth = can_pop(stat[i], i)
            if pop_depth >= 0:
                # Try to move it to the left hallway
                for h in range(align[i]-1,-1,-1):
                    if valid[h]:
                        if stat[1][h] == '.':
                            new_stat = [stat[0], [a for a in stat[1]], [a for a in stat[2]], [a for a in stat[3]], [a for a in stat[4]], [a for a in stat[5]]]
                            new_stat[0] += cost[stat[i][pop_depth]] * (1+pop_depth+abs(align[i]-h))
                            new_stat[1][h] = new_stat[i][pop_depth]
                            new_stat[i][pop_depth] = '.'

                            if new_stat[0] not in stat_queue.keys(): stat_queue[new_stat[0]] = []
                            stat_queue[new_stat[0]].append((new_stat[0],)+tuple([tuple([a for a in b]) for b in new_stat[1:]]))
                        else:
                            break
                
                # Try to move it to the right hallway
                for h in range(align[i]+1,11,1):
                    if valid[h]:
                        if stat[1][h] == '.':
                            new_stat = [stat[0], [a for a in stat[1]], [a for a in stat[2]], [a for a in stat[3]], [a for a in stat[4]], [a for a in stat[5]]]
                            new_stat[0] += cost[stat[i][pop_depth]] * (1+pop_depth+abs(align[i]-h))
                            new_stat[1][h] = new_stat[i][pop_depth]
                            new_stat[i][pop_depth] = '.'

                            if new_stat[0] not in stat_queue.keys(): stat_queue[new_stat[0]] = []
                            stat_queue[new_stat[0]].append((new_stat[0],)+tuple([tuple([a for a in b]) for b in new_stat[1:]]))
                        else:
                            break

    new_low_energy = sys.maxsize
    for energy_state in sorted(stat_queue):
        for stat in stat_queue[energy_state]:
            new_energy = find_true_void(stat)
            new_low_energy = min(new_energy, new_low_energy)

    return new_low_energy

ta = time.perf_counter()
with open(sys.argv[1]) as data:
    global_min_energy = sys.maxsize
    lines = data.readlines()

    room1 = (lines[2][3], lines[3][3], lines[4][3], lines[5][3])
    room2 = (lines[2][5], lines[3][5], lines[4][5], lines[5][5])
    room3 = (lines[2][7], lines[3][7], lines[4][7], lines[5][7])
    room4 = (lines[2][9], lines[3][9], lines[4][9], lines[5][9])

    hallway = tuple(list('.' * 11))

    stat = (0, hallway, room1, room2, room3, room4)

    energy = find_true_void(stat)

tb = time.perf_counter()
print(f"Kowalsky, Squid formation!: {energy} {(tb-ta):0.2f}")
