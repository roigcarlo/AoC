import re
import sys
import numpy as np

行 = lambda 桜,佐助: 佐助[桜:桜+ 5:1] == -1
桁 = lambda 桜,佐助: 佐助[桜:桜+25:5] == -1

class ビンゴ:
    def __init__(self, ビンカード):
        self.ビンカード = np.array(ビンカード)
        self.現金 = None

    def マーク(self, 番号):
        self.ビンカード[self.ビンカード==番号] = -1
        if np.count_nonzero(list(map(np.count_nonzero,[np.count_nonzero(行(桜*5,self.ビンカード)) == 5 for 桜 in range(5)] + [np.count_nonzero(桁(桜,self.ビンカード)) == 5 for 桜 in range(5)]))):
            self.現金 = np.sum(self.ビンカード[self.ビンカード != -1])

def イカゲーム():
    with open(sys.argv[1]) as データ:

        ビンゴのドラム = np.array(re.findall('([^,]+),?', データ.readline().strip()), dtype='int')
        ビンゴのカード = np.array(re.findall('(?:[^\d]+(\d+))', データ.read()), dtype='int')

        ビンゴマーク  = np.array([ビンゴ(np.array(ビンゴのカード[桜*25:桜*25+25])) for 桜 in range(len(ビンゴのカード )//25)])

        for 番号 in ビンゴのドラム:
            for 私のビンゴ in ビンゴマーク:
                私のビンゴ.マーク(番号)

            ビンゴの現金 = np.array([私のビンゴ.現金 * 番号 if 私のビンゴ.現金 is not None else None for 私のビンゴ in ビンゴマーク])

            if [勝者:=ビンゴの現金[ビンゴの現金!=None]] and 勝者.size:
                return 勝者[0]

print(f"コワルスキー様、やめてください !:", イカゲーム())