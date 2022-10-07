import sys

with open(sys.argv[1]) as data:
    player1 = int(data.readline()[28])-1
    player2 = int(data.readline()[28])-1

score1, score2 = 0, 0

dice = 0
roll = 0

while True:
    for _ in range(3):
        player1 = (player1 + dice + 1) % 10
        dice = (dice+1) % 1000
        roll = roll + 1

    score1 += player1+1

    if score1 > 999 or score2 > 999:
            break 

    for _ in range(3):
        player2 = (player2 + dice + 1) % 10
        dice = (dice+1) % 1000
        roll = roll + 1

    score2 += player2+1

    if score1 > 999 or score2 > 999:
        break

print("Kowalsky, fear the second half:", min(score1, score2) * roll)