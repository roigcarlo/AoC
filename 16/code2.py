import sys
import functools

def proc_sub_pack(bits, version):
    length_type, bits = bits[0], bits[1:]
    values = []
    if length_type == '0':
        length, bits = bits[:15],bits[15:]
        length = int(length, 2)
        current_size = len(bits)
        while length:
            sub_version, value, bits = read_packet(bits)
            length -= (current_size - len(bits))
            current_size = len(bits)
            version += sub_version
            values.append(value)
    else:
        length, bits = bits[:11],bits[11:]
        sub_packets = int(length, 2)
        for _ in range(sub_packets):
            sub_version, value, bits = read_packet(bits)
            version += sub_version
            values.append(value)

    return version, values, bits

def read_packet(bits):
    version, bits = bits[:3],bits[3:]
    typeid , bits = bits[:3],bits[3:]

    version, typeid = int(version, 2), int(typeid, 2)

    match typeid:
        case 0:
            version, values, bits = proc_sub_pack(bits, version)
            value = sum(values)
        case 1:
            version, values, bits = proc_sub_pack(bits, version)
            value = functools.reduce(lambda x,y: x*y, values)
        case 2:
            version, values, bits = proc_sub_pack(bits, version)
            value = min(values)
        case 3:
            version, values, bits = proc_sub_pack(bits, version)
            value = max(values)
        case 4:
            num = ''
            while bits[0] == '1':
                hex, bits = bits[:5][1:],bits[5:]
                num += hex
            hex, bits = bits[:5][1:],bits[5:]
            num += hex
            value = int(num, 2)
        case 5:
            version, values, bits = proc_sub_pack(bits, version)
            value = int(values[0] > values[1])
        case 6:
            version, values, bits = proc_sub_pack(bits, version)
            value = int(values[0] < values[1])
        case 7:
            version, values, bits = proc_sub_pack(bits, version)
            value = int(values[0] == values[1])
        case _:
            print("Bip bop bip ¡BOOOM!")

    return version, value, bits


with open(sys.argv[1]) as data:
    bits = data.readline().strip()

    bits = ''.join([bin(int(a, 16))[2:].zfill(4) for a in bits])

    version, value, bits = read_packet(bits)

    print("Kowalsky, bip bop bip:", value)