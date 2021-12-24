import sys
import time
import functools

cost   = {'A':1,'B':10,'C':100,'D':1000}
target = {1:'A',2:'B',3:'C',4:'D'}
dest   = {'A':1,'B':2,'C':3,'D':4}
valid  = [1,1,0,1,0,1,0,1,0,1,1]
align  = {1:2,2:4,3:6,4:8}

def pretty_print(stat):
    print("#"*13)
    print("#"+''.join(stat[1])+"#")
    print("###"+stat[2][0]+"#"+stat[3][0]+"#"+stat[4][0]+"#"+stat[5][0]+"###")
    print("  #"+stat[2][1]+"#"+stat[3][1]+"#"+stat[4][1]+"#"+stat[5][1]+"#  ")
    print("  #"+stat[2][2]+"#"+stat[3][2]+"#"+stat[4][2]+"#"+stat[5][2]+"#  ")
    print("  #"+stat[2][3]+"#"+stat[3][3]+"#"+stat[4][3]+"#"+stat[5][3]+"#  ")
    print("  "+"#"*9)
    print(f"  <<{stat[0]:5d}>> ({global_min['value']})")

@functools.cache
def can_push(room, rid):
    index = -1
    for i in range(len(room)):
        if room[i] == '.': index = i
        if room[i] != '.' and room[i] != target[rid]: index = -1
    return index

@functools.cache
def can_pop(room, rid):
    index = -1
    for i in range(len(room)):
        if room[i] != '.' and room[i] != target[rid]: return i
        if room[i] != '.' and room[i] == target[rid]: return -1 if all(e == room[i] for e in room[i:]) else i
    return index

@functools.cache
def find_true_void(stat):
    if stat[0] == '.'*11 and stat[1] == 'A'*4 and stat[2] == 'B'*4 and stat[3] == 'C'*4 and stat[4] == 'D'*4:
        return 0

    # Move the ampis from the hallway
    for i in range(11):
        if stat[0][i] != '.':
            room_dst = dest[stat[0][i]]
            
            pos_a, pos_b = i, align[room_dst]

            if pos_a > pos_b: pos_a, pos_b = pos_b, pos_a
            else: pos_a, pos_b = pos_a +1, pos_b + 1

            if stat[0][pos_a:pos_b] == '.'*abs(i-align[room_dst]):
                push_depth = can_push(stat[room_dst], room_dst)
                if push_depth != -1:
                    new_stat = [list(stat[0]), list(stat[1]), list(stat[2]), list(stat[3]), list(stat[4])]
                    move_cost = cost[stat[0][i]] * (1+push_depth+abs(align[room_dst]-i))
                    new_stat[room_dst][push_depth] = stat[0][i]
                    new_stat[0][i] = '.'

                    return move_cost + find_true_void(tuple([''.join(b) for b in new_stat]))

    # Initialize the queue of stats
    stat_queue = {}

    # Move ampis out of the rooms
    for i in range(1,5):
        pop_depth = can_pop(stat[i], i)
        if pop_depth >= 0:
            # Try to move it to the left hallway
            for h in range(align[i]-1,-1,-1):
                if valid[h]:
                    if stat[0][h] == '.':
                        new_stat = [list(stat[0]), list(stat[1]), list(stat[2]), list(stat[3]), list(stat[4])]
                        move_cost = cost[stat[i][pop_depth]] * (1+pop_depth+align[i]-h)
                        new_stat[0][h] = new_stat[i][pop_depth]
                        new_stat[i][pop_depth] = '.'

                        if move_cost not in stat_queue.keys(): stat_queue[move_cost] = []
                        stat_queue[move_cost].append(tuple([''.join(b) for b in new_stat]))
                    else:
                        break
            
            # Try to move it to the right hallway
            for h in range(align[i]+1,11,1):
                if valid[h]:
                    if stat[0][h] == '.':
                        new_stat = [list(stat[0]), list(stat[1]), list(stat[2]), list(stat[3]), list(stat[4])]
                        move_cost = cost[stat[i][pop_depth]] * (1+pop_depth+h-align[i])
                        new_stat[0][h] = new_stat[i][pop_depth]
                        new_stat[i][pop_depth] = '.'

                        if move_cost not in stat_queue.keys(): stat_queue[move_cost] = []
                        stat_queue[move_cost].append(tuple([''.join(b) for b in new_stat]))
                    else:
                        break

    new_low_energy = sys.maxsize
    for energy_state in sorted(stat_queue):
        for stat in stat_queue[energy_state]:
            new_energy = energy_state + find_true_void(stat)
            new_low_energy = min(new_energy, new_low_energy)

    return new_low_energy

ta = time.perf_counter()
with open(sys.argv[1]) as data:
    global_min_energy = sys.maxsize
    lines = data.readlines()

    room1 = lines[2][3]+lines[3][3]+lines[4][3]+lines[5][3]
    room2 = lines[2][5]+lines[3][5]+lines[4][5]+lines[5][5]
    room3 = lines[2][7]+lines[3][7]+lines[4][7]+lines[5][7]
    room4 = lines[2][9]+lines[3][9]+lines[4][9]+lines[5][9]

    hallway = '.'*11

    stat = (hallway, room1, room2, room3, room4)

    energy = find_true_void(stat)

tb = time.perf_counter()

print(f"Kowalsky, Squid formation!: {energy} {(tb-ta):0.2f}")
