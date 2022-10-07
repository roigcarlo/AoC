import sys

x, z, a = 0, 0, 0

with open(sys.argv[1]) as data:
    for l in data:
        direction = [type(val) for type,val in zip([str, int],l.strip().split(" "))]

        match direction:
            case ["forward", val]:
                x += val
                z += val * a
            case ["up", val]:
                a -= val
            case ["down", val]:
                a += val
            case _:
                print(f"Order not acknowledge: {direction=}")

print(f"Admiral Kowalsky, we reached {x*z}!")
