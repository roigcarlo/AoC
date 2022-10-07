# Dive! #

## Featuring... Match expressions! ##

I wanted to test match expressions from python 3.10

A match expression matches the expression you pass:

```python

match "1-2-3".split('-'):
    case a,b,c:
        print(f"{a=},{b=},{c=}")
    case _:
        print(f"Fail")
```

will print:

```
a=1,b=2,c=3
```

Isn't it nice?