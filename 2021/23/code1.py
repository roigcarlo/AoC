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
    print("  "+"#"*9)
    print(f"  <<{stat[0]:5d}>> ({global_min['value']})")

@functools.cache
def find_true_void(stat):
    if stat[0] >= global_min['value']:
        return sys.maxsize

    if stat[1] == ('.','.','.','.','.','.','.','.','.','.','.') and stat[2] ==('A', 'A') and stat[3] == ('B', 'B') and stat[4] == ('C', 'C') and stat[5] == ('D', 'D'):
        if stat[0] < global_min['value']:
            global_min['value'] = stat[0]
        return stat[0]

    energies = []
    stat_queue = {}

    # Move the ampis from the hallway
    found_correct_pos = False
    for i in range(11):
        if stat[1][i] != '.':
            room_dst = dest[stat[1][i]]
            
            pos_a, pos_b = i, align[room_dst]
            if pos_a > pos_b:
                pos_a, pos_b = pos_b, pos_a
            else:
                pos_a += 1
                pos_b += 1
            if stat[1][pos_a:pos_b] == tuple(['.' for _ in range(abs(i-align[room_dst]))]):
                if stat[room_dst][1] == '.':
                    new_stat = [stat[0], [a for a in stat[1]], [a for a in stat[2]], [a for a in stat[3]], [a for a in stat[4]], [a for a in stat[5]]]
                    new_stat[0] += cost[stat[1][i]] * (2+abs(align[room_dst]-i))
                    new_stat[room_dst][1] = stat[1][i]
                    new_stat[1][i] = '.'
                    if new_stat[0] not in stat_queue.keys(): stat_queue[new_stat[0]] = []
                    stat_queue[new_stat[0]].append((new_stat[0],)+tuple([tuple([a for a in b]) for b in new_stat[1:]]))
                    found_correct_pos = True
                    break
                elif stat[room_dst][0] == '.' and stat[room_dst][1] == target[room_dst]:
                    new_stat = [stat[0], [a for a in stat[1]], [a for a in stat[2]], [a for a in stat[3]], [a for a in stat[4]], [a for a in stat[5]]]
                    new_stat[0] += cost[stat[1][i]] * (1+abs(align[room_dst]-i))
                    new_stat[room_dst][0] = stat[1][i]
                    new_stat[1][i] = '.'
                    if new_stat[0] not in stat_queue.keys(): stat_queue[new_stat[0]] = []
                    stat_queue[new_stat[0]].append((new_stat[0],)+tuple([tuple([a for a in b]) for b in new_stat[1:]]))
                    found_correct_pos = True
                    break

    if not found_correct_pos:
    # Move ampis out of the rooms
        for i in range(5,1,-1):
            # Check if the ampi in the top has to move
            if stat[i][0] != '.' and (stat[i][1] != target[i] or stat[i][0] != target[i]):
                # Try to move it to another room
                can_into_room = False
                for j in range(2,6):
                    if i != j and stat[i][0] == target[j]:
                        # To the top
                        if stat[j][0] == '.' and stat[j][1] == target[j]:
                            pos_a, pos_b = i, j
                            if pos_a > pos_b:
                                pos_a, pos_b = pos_b, pos_a
                            else:
                                pos_a += 1
                                pos_b += 1

                            if stat[1][pos_a:pos_b] == tuple(['.' for _ in range(abs(align[i]-align[j]))]):
                                new_stat = [stat[0], [a for a in stat[1]], [a for a in stat[2]], [a for a in stat[3]], [a for a in stat[4]], [a for a in stat[5]]]
                                new_stat[0] += cost[stat[i][0]] * (2+abs(align[i]-align[j]))
                                new_stat[j][0] = new_stat[i][0]
                                new_stat[i][1] = '.'
                                if new_stat[0] not in stat_queue.keys(): stat_queue[new_stat[0]] = []
                                stat_queue[new_stat[0]].append((new_stat[0],)+tuple([tuple([a for a in b]) for b in new_stat[1:]]))
                                can_into_room = True
                                break

                        # To the bot
                        if stat[j][1] == '.':
                            pos_a, pos_b = i, j
                            if pos_a > pos_b:
                                pos_a, pos_b = pos_b, pos_a
                            else:
                                pos_a += 1
                                pos_b += 1

                            if stat[1][pos_a:pos_b] == tuple(['.' for _ in range(abs(align[i]-align[j]))]):
                                new_stat = [stat[0], [a for a in stat[1]], [a for a in stat[2]], [a for a in stat[3]], [a for a in stat[4]], [a for a in stat[5]]]
                                new_stat[0] += cost[stat[i][0]] * (3+abs(align[i]-align[j]))
                                new_stat[j][1] = new_stat[i][0]
                                new_stat[i][1] = '.'
                                if new_stat[0] not in stat_queue.keys(): stat_queue[new_stat[0]] = []
                                stat_queue[new_stat[0]].append((new_stat[0],)+tuple([tuple([a for a in b]) for b in new_stat[1:]]))
                                can_into_room = True

                if not can_into_room:
                # Try to move it to the left hallway
                    for h in range(align[i]-1,-1,-1):
                        if valid[h]:
                            if stat[1][h] == '.':
                                new_stat = [stat[0], [a for a in stat[1]], [a for a in stat[2]], [a for a in stat[3]], [a for a in stat[4]], [a for a in stat[5]]]
                                new_stat[0] += cost[stat[i][0]] * (1+abs(align[i]-h))
                                new_stat[1][h] = new_stat[i][0]
                                new_stat[i][0] = '.'
                                if new_stat[0] + cost[stat[i][0]] * (1+abs(align[i]-h)) + cost[stat[i][0]] * (1+abs(align[dest[stat[i][0]]]-h)) < global_min['value']:
                                    if new_stat[0] not in stat_queue.keys(): stat_queue[new_stat[0]] = []
                                    stat_queue[new_stat[0]].append((new_stat[0],)+tuple([tuple([a for a in b]) for b in new_stat[1:]]))
                            else:
                                break
                    
                    # Try to move it to the right hallway
                    for h in range(align[i]+1,11,1):
                        if valid[h]:
                            if stat[1][h] == '.':
                                new_stat = [stat[0], [a for a in stat[1]], [a for a in stat[2]], [a for a in stat[3]], [a for a in stat[4]], [a for a in stat[5]]]
                                new_stat[0] += cost[stat[i][0]] * (1+abs(align[i]-h))
                                new_stat[1][h] = new_stat[i][0]
                                new_stat[i][0] = '.'
                                if new_stat[0] + cost[stat[i][0]] * (1+abs(align[i]-h)) + cost[stat[i][0]] * (1+abs(align[dest[stat[i][0]]]-h)) < global_min['value']:
                                    if new_stat[0] not in stat_queue.keys(): stat_queue[new_stat[0]] = []
                                    stat_queue[new_stat[0]].append((new_stat[0],)+tuple([tuple([a for a in b]) for b in new_stat[1:]]))
                            else:
                                break

            # Check if the ampi in the bottom has to move
            if stat[i][0] == '.' and stat[i][1] != '.' and stat[i][1] != target[i]:
                # Try to move it to another room
                can_into_room = False
                for j in range(2,6):
                    if i != j and stat[i][1] == target[j]:
                        # To the top
                        if stat[j][0] == '.' and stat[j][1] == target[j]:
                            pos_a, pos_b = i, j
                            if pos_a > pos_b:
                                pos_a, pos_b = pos_b, pos_a
                            else:
                                pos_a += 1
                                pos_b += 1

                            if stat[1][pos_a:pos_b] == tuple(['.' for _ in range(abs(align[i]-align[j]))]):
                                new_stat = [stat[0], [a for a in stat[1]], [a for a in stat[2]], [a for a in stat[3]], [a for a in stat[4]], [a for a in stat[5]]]
                                new_stat[0] += cost[stat[i][1]] * (5+abs(align[i]-align[j]))
                                new_stat[j][0] = new_stat[i][1]
                                new_stat[i][1] = '.'
                                if new_stat[0] not in stat_queue.keys(): stat_queue[new_stat[0]] = []
                                stat_queue[new_stat[0]].append((new_stat[0],)+tuple([tuple([a for a in b]) for b in new_stat[1:]]))
                                can_into_room = True
                        # To the bot
                        if stat[j][1] == '.':
                            pos_a, pos_b = i, j
                            if pos_a > pos_b:
                                pos_a, pos_b = pos_b, pos_a
                            else:
                                pos_a += 1
                                pos_b += 1

                            if stat[1][pos_a:pos_b] == tuple(['.' for _ in range(abs(align[i]-align[j]))]):
                                new_stat = [stat[0], [a for a in stat[1]], [a for a in stat[2]], [a for a in stat[3]], [a for a in stat[4]], [a for a in stat[5]]]
                                new_stat[0] += cost[stat[i][1]] * (6+abs(align[i]-align[j]))
                                new_stat[j][1] = new_stat[i][1]
                                new_stat[i][1] = '.'
                                if new_stat[0] not in stat_queue.keys(): stat_queue[new_stat[0]] = []
                                stat_queue[new_stat[0]].append((new_stat[0],)+tuple([tuple([a for a in b]) for b in new_stat[1:]]))
                                can_into_room = True

                if not can_into_room:
                    # Try to move it to the left hallway
                    for h in range(align[i]-1,-1,-1):
                        if valid[h]:
                            if stat[1][h] == '.':
                                new_stat = [stat[0], [a for a in stat[1]], [a for a in stat[2]], [a for a in stat[3]], [a for a in stat[4]], [a for a in stat[5]]]
                                new_stat[0] += cost[stat[i][1]] * (2+abs(align[i]-h))
                                new_stat[1][h] = new_stat[i][1]
                                new_stat[i][1] = '.'
                                if new_stat[0] + cost[stat[i][1]] * (2+abs(align[i]-h)) + cost[stat[i][1]] * (1+abs(align[dest[stat[i][1]]]-h)) < global_min['value']:
                                    if new_stat[0] not in stat_queue.keys(): stat_queue[new_stat[0]] = []
                                    stat_queue[new_stat[0]].append((new_stat[0],)+tuple([tuple([a for a in b]) for b in new_stat[1:]]))
                            else:
                                break

                    # Try to move it to the right hallway
                    for h in range(align[i]+1,11,1):
                        if valid[h]:
                            if stat[1][h] == '.':
                                new_stat = [stat[0], [a for a in stat[1]], [a for a in stat[2]], [a for a in stat[3]], [a for a in stat[4]], [a for a in stat[5]]]
                                new_stat[0] += cost[stat[i][1]] * (2+abs(align[i]-h))
                                new_stat[1][h] = new_stat[i][1]
                                new_stat[i][1] = '.'
                                if new_stat[0] + cost[stat[i][1]] * (2+abs(align[i]-h)) + cost[stat[i][1]] * (1+abs(align[dest[stat[i][1]]]-h)) < global_min['value']:
                                    if new_stat[0] not in stat_queue.keys(): stat_queue[new_stat[0]] = []
                                    stat_queue[new_stat[0]].append((new_stat[0],)+tuple([tuple([a for a in b]) for b in new_stat[1:]]))
                            else:
                                break

    new_low_energy = sys.maxsize
    for energy_state in sorted(stat_queue):
        for stat in stat_queue[energy_state]:
            if stat[0] < global_min['value']:
                new_energy = find_true_void(stat)
                global_min['recursions'] += 1
                global_min['value'] = min(new_energy, global_min['value'])
                new_low_energy = min(new_energy, new_low_energy)

    return new_low_energy

ta = time.perf_counter()
with open(sys.argv[1]) as data:
    global_min_energy = sys.maxsize
    lines = data.readlines()

    room1 = (lines[2][3], lines[5][3])
    room2 = (lines[2][5], lines[5][5])
    room3 = (lines[2][7], lines[5][7])
    room4 = (lines[2][9], lines[5][9])

    hallway = tuple(list('.' * 11))

    stat = (0, hallway, room1, room2, room3, room4)

    energy = find_true_void(stat)

tb = time.perf_counter()
print(f"Kowalsky, Squid formation!: {energy} {(tb-ta):0.2f}")
