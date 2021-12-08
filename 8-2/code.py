import re
import sys
import time

if __name__ == '__main__':
    ta = time.perf_counter()
    with open(sys.argv[1]) as data:
        digits = 0
        for l in data:
            encode, decode = map(lambda x: re.findall('(\w+)(?:\s|$)', x), l.split('|'))

            s2 = [set([c for c in word]) for word in encode if len(word) == 2]
            s3 = [set([c for c in word]) for word in encode if len(word) == 3]
            s4 = [set([c for c in word]) for word in encode if len(word) == 4]
            s5 = [set([c for c in word]) for word in encode if len(word) == 5]
            s6 = [set([c for c in word]) for word in encode if len(word) == 6]
            s7 = [set([c for c in word]) for word in encode if len(word) == 7]

            i5 = s5[0].intersection(*s5[1:])
            i6 = s6[0].intersection(*s6[1:])

            nixie_map = ['N'] * 7

            # 0:
            segment = s3[0]-s2[0]
            nixie_map[0] = segment.pop()

            # 3:
            segment = i5-i6
            nixie_map[3] = segment.pop()

            # 6:
            segment = i5 - set([nixie_map[0], nixie_map[3]])
            nixie_map[6] = segment.pop()

            # 1:
            segment = i6 - i5 - s2[0]
            nixie_map[1] = segment.pop()

            # 4:
            segment = s7[0] - s2[0] - i5 - set([nixie_map[1]])
            nixie_map[4] = segment.pop()

            # 2:
            segment = s4[0] - i5 - i6
            nixie_map[2] = segment.pop()

            # 5:
            segment = set(['a','b','c','d','e','f','g']) - set([nixie_map[i] for i in range(7) if nixie_map[i] != 'N'])
            nixie_map[5] = segment.pop()

            number = ''
            for d in decode:
                match len(d):
                    case 2:
                        number += '1'
                    case 3:
                        number += '7'
                    case 4:
                        number += '4'
                    case 5:
                        if   nixie_map[2] not in d:
                            number += '5'
                        elif nixie_map[5] not in d:
                            number += '2'
                        else:
                            number += '3'
                    case 6:
                        if   nixie_map[2] not in d:
                            number += '6'
                        elif nixie_map[4] not in d:
                            number += '9'
                        else:
                            number += '0'
                    case 7:
                        number += '8'
            
            digits += int(number)
                
    tb = time.perf_counter()
    print("Kowlasky, today it's enough solving this:", digits, (tb-ta)*1000)