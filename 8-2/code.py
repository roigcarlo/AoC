import re
import sys
import time

if __name__ == '__main__':
    ta = time.perf_counter()
    with open(sys.argv[1]) as data:
        digits = 0
        for l in data:
            encode, decode = map(lambda x: re.findall('(\w+)(?:\s|$)', x), l.split('|'))

            s = [[set([c for c in word]) for word in encode if len(word) == r] for r in range(3,8)]

            # s3 = [set([c for c in word]) for word in encode if len(word) == 3]
            # s4 = [set([c for c in word]) for word in encode if len(word) == 4]
            # s5 = [set([c for c in word]) for word in encode if len(word) == 5]
            # s6 = [set([c for c in word]) for word in encode if len(word) == 6]
            # s7 = [set([c for c in word]) for word in encode if len(word) == 7]

            i5 = s[2][0].intersection(*s[2][1:])
            i6 = s[3][0].intersection(*s[3][1:])

            n4 = set(s[4][0] - i5 - i6 - s[0][0]).pop()
            n2 = set(s[1][0] - i5 - i6).pop()
            n5 = set(s[1][0].intersection(i6,s[0][0])).pop()

            nixie_map = {2:(lambda x:'1'), 3:(lambda x:'7'), 4:(lambda x:'4'), 7:(lambda x:'8'), 5:(lambda x:('5',('2','3')[n5 in x])[n2 in x]), 6:(lambda x:('6',('9','0')[n4 in x])[n2 in x])}

            digits += int(''.join([nixie_map[len(d)](d) for d in decode]))
                
    tb = time.perf_counter()
    print("Kowlasky, today it's enough solving this:", digits, (tb-ta)*1000)