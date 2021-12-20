# Beacon Scanner #

## Featuring... $#!@ ! ##

What a beautiful and inter... NO. 

But after spending 24h with the puzzle, here is how I solved it.

Instead of using brute force I created a hash-like structure for every scanner
such that evey canner holds the distances of every pair of nodes that see.

All scanners that share 66 distances in common do overlap. The 66 represents all possible
combinations of the 12 different overlapping points `N*(N-1)/2`.

Once all the overlapping scanners are found, we need to deduce the transformations between
the spaces they see.

For every pair of points seen from two different spaces (that we can access directly using the distances)
we can deduce the transformation used from going from A to B. The problem is that we don't know which of the
pair of the points transforms to which of the other pair.

I am not 100% sure but I think that, by the way they are constructed, we can ensure that at least we are gonna hit
half of the test, but even so, we are sure that if we do not hit, the wring transformation will appear at most
once (very likely, in a real case we should increase that). That means that once we find 2 points with the same
transformation, that one is the correct.

With all the transformations and overlapping we can calculate the chains of transformation from one space to another and its DONE.
