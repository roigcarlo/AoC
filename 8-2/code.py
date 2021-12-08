import re
import sys
import time

if __name__ == '__main__':
    ta = time.perf_counter()
    with open(sys.argv[1]) as data:
        digits = 0
        for l in data:
            encode, decode = map(lambda x: re.findall('(\w+)(?:\s|$)', x), l.split('|'))

            s3 = [set([c for c in word]) for word in encode if len(word) == 3]
            s4 = [set([c for c in word]) for word in encode if len(word) == 4]
            s5 = [set([c for c in word]) for word in encode if len(word) == 5]
            s6 = [set([c for c in word]) for word in encode if len(word) == 6]
            s7 = [set([c for c in word]) for word in encode if len(word) == 7]

            i5 = s5[0].intersection(*s5[1:])
            i6 = s6[0].intersection(*s6[1:])

            # 4:
            segment = s7[0] - i5 - i6 - s3[0]
            n4 = segment.pop()

            # 2:
            segment = s4[0] - i5 - i6
            n2 = segment.pop()

            # 5:
            segment = s4[0].intersection(i6,s3[0])
            n5 = segment.pop()

            number = ''
            for d in decode:
                match len(d):
                    case 2: number += '1'
                    case 3: number += '7'
                    case 4: number += '4'
                    case 5:
                        if   n2 not in d: number += '5'
                        elif n5 not in d: number += '2'
                        else:             number += '3'
                    case 6:
                        if   n2 not in d: number += '6'
                        elif n4 not in d: number += '9'
                        else:             number += '0'
                    case 7: number += '8'
            
            digits += int(number)
                
    tb = time.perf_counter()
    print("Kowlasky, today it's enough solving this:", digits, (tb-ta)*1000)