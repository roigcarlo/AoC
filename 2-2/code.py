import sys

x, z, a = 0,0,0

with open(sys.argv[1]) as data:
    for l in data:
        direction = l.strip().split(" ")

        match direction:
            case ["forward", value]:
                x += int(value)
                z += int(value) * a
            case ["up", value]:
                a -= int(value)
            case ["down", value]:
                a += int(value)
            case _:
                print(f"Order not acknowledge: {direction=}")

print(f"Admiral Kowalsky, we reached {x*z}!")
