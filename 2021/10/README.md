# Syntax Scoring #

## Featuring... Recursive Regex! ##

Since I lack regularity and stability in my life I am naturally fascinated by regular and stable constructs as regular expressions are.

In this particular case we are using the `regex` module instead of the most known `re` because the later lacks a marvellous feature: Recursivity!

What is recursivity you my ask my little penguin? recursivity is recursivity.

The trick to solve the problem like a boss is this expression:

```regex
((?:\(|\[|\{|<)(?:(?R)*)(?:\)|\]|\}|>))
```

What this does? Basically turns pairs of matching open and close symbols outside into a list of pairs:

```
(<>[])<>((
```

becomes

```
()<>[]<>
```

Such fantastic. Believe me, you need recursivity for that.

Is the code faster than traversing the string with a stack? No, it is 40 times slower. Is it cooler? **A Lot**.
