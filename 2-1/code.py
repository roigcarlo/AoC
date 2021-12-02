import sys

x = 0
z = 0

with open(sys.argv[1]) as data:
    for l in data:
        direction = l.split(" ")

        match direction:
            case ["forward", value]:
                x += int(value)
            case ["up", value]:
                z -= int(value)
            case ["down", value]:
                z += int(value)

print(f"Admiral Kowalsky, we reached {x*z}!")
