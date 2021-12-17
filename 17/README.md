# Trick Shot #

## Featuring... Github Copilot! ##

If you are reading this, you may have notice how lazy I am some days. Well today I am *SPECIALLY* lazy, so I did not write any code (well some of it but very few)

Today's puzzle has been mostly coded by [Github's Copilot](https://copilot.github.com/)
which is a tool powered by OpenAI's GPT3 that assist programmers in the creation of codes.

<p align="center">
  <img width=50% src="https://github.githubassets.com/images/icons/copilot/cp-head-square.png" />
</p>

All functions in the puzzles today have been written as textual description and sligtly tweaked (the lines with comments have been rewritten or changed by me).

The main code is written mostly by me:

```Python
''' main ''' # This one is mostly written by me
def main():
    with open(sys.argv[1]) as data:
        ta = data.readline().strip()

        m = re.findall('(-?(?:\d)+)', ta)
        max_y = calculate_max_y(int(m[0]), int(m[2]), int(m[1]), int(m[3]))
        print("Kowalsky, let the AI guide you:", max_y)

if __name__ == '__main__':
    main()
```

Then for example I declared a function stating:

```Python
''' A function that takes the coordinates of a square and an initial velocity. Then the function 
    does an infinite loop applying this rules:

    The probe's x position increases by its x velocity.
    The probe's y position increases by its y velocity.
    The function stores the maximum y value.
    The probes velocity is updated.
    If the probe is beyond the square, sets the maxmium y position to minus one and breaks the loop.
    If the probe is inside the square, breaks the loop

Finally, the function returns the maximum y value.
'''
def calculate_max_y_and_update_velocity(x1, y1, x2, y2, vx, vy):
```

Which produced The code below. Note that the description has to be very clear and I needed to iterate over it several times to produce the right solution between the given proposals:

```Python
max_y = -1
x = 0
y = 0
while True:
    x = x + vx
    y = y + vy
    if is_inside(x, y, x1, y1, x2, y2):
        break
    if is_beyond_the_square(x1, y1, x2, y2, x, y, vx):
        max_y = -1
        break
    vx, vy = update_velocity(vx, vy)
    max_y = max(max_y, y)
return max_y
```

Not all the codes were perfect and some of them would have been very dificult to 
descrive. For example the ranges in this function were ultimately defined by me

```Python
''' A function that given a square calculates which is the pair of x,y velocities maximizes the output of the 
function calculate_max_y_and_update_velocity in the range of x velocities [-x1, x1] and y velocities [-y1, y1]
'''
def calculate_max_y(x1, y1, x2, y2):
    ax1, ay1, ax2, ay2 = turn_into_positive_numbers(x1, y1, x2, y2)
    max_y = -1
    # I tunned the ranges so it converges faster.
    for x in range(0, max(ax1+1,ax2+1)): 
        # I tunned the ranges so it converges faster.                                               
        for y in range(-max(ay1+1,ay2+1,ax1+1,ax2+1), max(ay1+1,ay2+1,ax1+1,ax2+1)):
            max_y = max(max_y, calculate_max_y_and_update_velocity(x1, y1, x2, y2, x, y))
    return max_y
```

But copilot was able to deduce the correct ranges in the second part:

```Python
''' A function that given a square calculates how many x,y velocities makes the probe land inside the square'''
# Note: it also took the info of the previous prolem to correctly guess the ranges
def calculate_how_many_fall_inside(x1, y1, x2, y2):
    ax1, ay1, ax2, ay2 = turn_into_positive_numbers(x1, y1, x2, y2)
    count = 0
    for x in range(0, max(ax1+1,ax2+1)):
        for y in range(-max(ay1+1,ay2+1,ax1+1,ax2+1), max(ay1+1,ay2+1,ax1+1,ax2+1)):
            if check_if_lands_inside(x1, y1, x2, y2, x, y):
                count += 1
    return count
```

My conclusions after using it. It is more usefull that it may seem at first glance, and by fare much more usefull than it was some months ago!

Disclaimer: This README.md has been written by a human. Maybe.