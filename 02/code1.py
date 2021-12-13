import sys

x, z = 0, 0

with open(sys.argv[1]) as data:
    for l in data:
        direction = [type(val) for type,val in zip([str, int],l.strip().split(" "))]

        match direction:
            case ["forward", val]:
                x += val
            case ["up", value]:
                z -= val
            case ["down", value]:
                z += val

print(f"Admiral Kowalsky, we reached {x*z}!")
