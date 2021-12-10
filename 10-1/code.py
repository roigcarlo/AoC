import sys
import regex
import itertools

print("Kowalsky, let's do some flexing!:", sum(list(itertools.chain(*list(itertools.chain(*[[list(filter(lambda x: x != 0,[(0,{')':3  ,']':57  ,'}':1197,'>':25137}[s[-1]])[{'(':')','[':']' ,'{':'}' ,'<':'>'}[s[0]]!=s[-1]] for s in m.captures(1)])) for m in regex.finditer('((?:\(|\[|\{|<)(?:(?R)*)(?:\)|\]|\}|>))', l.strip())] for l in open(sys.argv[1])]))))))