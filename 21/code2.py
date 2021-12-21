import sys
import time
import itertools
import functools

multimul = {3:1,4:3,5:6,6:7,7:6,8:3,9:1}
manifold = list(itertools.product([3,4,5,6,7,8,9],repeat=2))

@functools.lru_cache(maxsize=None)
def play(player1, player2, score1, score2):
    if score1 > 20: return 1, 0
    if score2 > 20: return 0, 1

    wins1, wins2 = 0, 0
    
    for garden in manifold:
        n_player1 = (player1 + garden[0]) % 10
        n_player2 = (player2 + garden[1]) % 10
        n_score1  = score1 + n_player1 + 1
        n_score2  = score2 + n_player2 + 1
        wins_garden = play(n_player1, n_player2, n_score1, n_score2)
        wins1 += wins_garden[0] * multimul[garden[0]] * (multimul[garden[1]],1)[n_score1 > 20]
        wins2 += wins_garden[1] * multimul[garden[0]] * multimul[garden[1]]

    return wins1, wins2

ta = time.perf_counter()
with open(sys.argv[1]) as data:
    player1 = int(data.readline()[28])-1
    player2 = int(data.readline()[28])-1

wins1, wins2 = play(player1, player2, 0, 0)
tb = time.perf_counter()

# 7 is a maigc number. Thx to the wisdom of the white tower we know why.
print(f"Kowalsky, fear the second half: {max(wins1//7, wins2)}, {(tb-ta)*1000:0.2f}ms")