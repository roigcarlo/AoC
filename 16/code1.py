import sys

def read_packet(bits):
    version, bits = bits[:3],bits[3:]
    typeid , bits = bits[:3],bits[3:]

    version, typeid = int(version, 2), int(typeid, 2)

    if typeid == 4:
        num = ''
        while bits[0] == '1':
            hex, bits = bits[:5][1:],bits[5:]
            num += hex
        hex, bits = bits[:5][1:],bits[5:]
        num += hex
    else:
        length_type, bits = bits[0], bits[1:]
        if length_type == '0':
            length, bits = bits[:15],bits[15:]
            length = int(length, 2)
            current_size = len(bits)
            while length:
                sub_version, bits = read_packet(bits)
                length -= (current_size - len(bits))
                current_size = len(bits)
                version += sub_version
        else:
            length, bits = bits[:11],bits[11:]
            sub_packets = int(length, 2)
            for _ in range(sub_packets):
                sub_version, bits = read_packet(bits)
                version += sub_version

    return version, bits


with open(sys.argv[1]) as data:
    bits = data.readline().strip()

    bits = ''.join([bin(int(a, 16))[2:].zfill(4) for a in bits])

    version, bits = read_packet(bits)

    print("Kowalsky, bip bop bip:", version)