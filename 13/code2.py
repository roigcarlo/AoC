import re
import sys
import time
import curses
import numpy as np

class curses_screen:
    def __enter__(self):
        self.stdscr = curses.initscr()
        curses.cbreak()
        curses.noecho()
        curses.start_color()
        curses.use_default_colors()
        self.stdscr.nodelay(1)
        self.stdscr.keypad(1)
        SCREEN_HEIGHT, SCREEN_WIDTH = self.stdscr.getmaxyx()
        return self.stdscr

    def __exit__(self,a,b,c):
        curses.nocbreak()
        self.stdscr.keypad(0)
        curses.echo()
        curses.endwin()

# Calculate
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
                max_x = warp_speed - 1

            if fold_space_and_time == 'y':
                black_holes = {(bh[0], (bh[1],max_y-bh[1])[bh[1]>warp_speed]) for bh in black_holes}
                max_y = warp_speed - 1

the_message_written_in_the_darkness_of_space = np.full((max_x+3, max_y+3), ' ', dtype='object')
for bh in black_holes:
    the_message_written_in_the_darkness_of_space[bh[0]+1, bh[1]+1] = '█'

# Display
with curses_screen() as stdscr:

    for i in range(0, curses.COLORS):
        curses.init_pair(i + 1, i, -1)

    color_matrix = np.zeros((6,6,6), dtype='int')
    for i in range(6):
        for j in range(6):
            for k in range(6):
                color_matrix[i,j,k] = 17+i*36+j*6+k

    color_matrix = np.repeat(color_matrix,7,axis=2)
    win = curses.newwin(max_x+2, max_y+2, 0, 0)

    ss, cx, cy, cz = 0,0,0,0

    while True:
        stdscr.addstr(0, 0, "Kowalsky, look at the stars and tell me what you see!:")
        
        for y in range(max_y+3):
            message = ''.join(np.asarray(the_message_written_in_the_darkness_of_space.T[y]))
            for m in range(len(message)):
                stdscr.addstr(y+1, m, message[m], curses.color_pair(
                    color_matrix[
                        (cz)%10     if (cz)%10     <  6 else 10-cz,
                        (cy+y-3)%10 if (cy+y-3)%10 <  6 else 10-((cy+y-3)%10),
                        (cx+m  )%78 if (cx+m  )%78 < 40 else 78-((cx+m  )%78)
                    ]
                ))

        if stdscr.getch() == ord('q'):
            break

        cx = (cx+1) % 78
        cy = (cy+1) % 10
        cz = (cz+1) % 10

        ss = (ss+1) % 100 

        stdscr.refresh()
        time.sleep(1/18)