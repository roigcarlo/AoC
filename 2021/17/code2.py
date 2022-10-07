import sys  # I wrote the import
import re   # I wrote the import

''' A function that given the coordinates of a square and a point with coordinates x and y decides if x and y falls inside the square '''
def is_inside(x, y, x1, y1, x2, y2):
    if x1 <= x <= x2 and y1 <= y <= y2:
        return True
    else:
        return False

''' A function that given a x,y velocity updates it following this rules:
    The probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
    The probe's y velocity decreases by 1.
'''
def update_velocity(vx, vy):
    if vx > 0:
        vx -= 1
    elif vx < 0:
        vx += 1
    else:
        vx = 0
    vy -= 1
    return vx, vy

''' A function that given a square coordinates a position and a direction returns true if:
    the direction is negative or zero and the position x is less than the square's x1 or
    the direction is positive or zero and the position x is greater than the square's x2 or
    the position y is less than the square's y1

    and false otherwise
'''
def is_beyond_the_square(x1, y1, x2, y2, x, y, vx):
    if vx < 0:
        if x < x1:
            return True
    elif vx > 0:
        if x > x2:
            return True
    if y < y1:
        return True
    return False

''' A function that takes the coordinates of a square and an initial velocity. Then the function does an infinite loop applying this rules:

    The probe's x position increases by its x velocity.
    The probe's y position increases by its y velocity.
    The probes velocity is updated.
    If the probe is beyond the square, sets the maxmium y position to minus one and breaks the loop.
    If the probe is inside the square, return one
'''
def check_if_lands_inside(x1, y1, x2, y2, vx, vy):
    initvx = vx
    initvy = vy
    x = 0
    y = 0
    while True:
        x = x + vx
        y = y + vy
        if is_inside(x, y, x1, y1, x2, y2):
            return True
        if is_beyond_the_square(x1, y1, x2, y2, x, y, vx):
            return False
        vx, vy = update_velocity(vx, vy)

''' A function that given a square calculates how many x,y velocities makes the probe land inside the square'''
# Note: It tooks the info of the previous prolem to correctly guess the ranges
def calculate_how_many_fall_inside(x1, y1, x2, y2):
    ax1, ay1, ax2, ay2 = turn_into_positive_numbers(x1, y1, x2, y2)
    count = 0
    for x in range(0, max(ax1+1,ax2+1)):
        for y in range(-max(ay1+1,ay2+1,ax1+1,ax2+1), max(ay1+1,ay2+1,ax1+1,ax2+1)):
            if check_if_lands_inside(x1, y1, x2, y2, x, y):
                count += 1
    return count

''' A functon that turns four numbers into positive mumbers and returns them '''
def turn_into_positive_numbers(x1, y1, x2, y2):
    x1 = abs(x1)
    x2 = abs(x2)
    y1 = abs(y1)
    y2 = abs(y2)
    return x1, y1, x2, y2

''' main ''' # This one is mostly written by me
def main():
    with open(sys.argv[1]) as data:
        ta = data.readline().strip()

        m = re.findall('(-?(?:\d)+)', ta)
        max_y = calculate_how_many_fall_inside(int(m[0]), int(m[2]), int(m[1]), int(m[3]))
        print("Kowalsky, let the AI guide you:", max_y)

if __name__ == '__main__':
    main()