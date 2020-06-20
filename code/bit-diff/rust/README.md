# Bit Diff

Given two integers, determine if those integers
are different by a single bit.

## Solution

We can do an XOR comparison of the integers to get
the bits that are different between the two numbers,
as represented by another integer. We can loop over
that integer's bits and check if there is more than one
bit that is nonzero.

E.g. given integer (0111 1101) and (1101 1100), the XOR of
that integer is (1010 0001). We can check whether the
right-most bit to see if it's one by doing a bitwise AND
comparing to 1 (0000 0001), and seeing if it equals 1.

This first time, yes it does equal 1. That tells us that the
right-most bit is 1. To test if any other bits are on, we need
to right-shift the integer, and compare again and again until
we're done (we'll be done when the number is 0).

Right shifting, we get (0101 0000). That comparison doesn't show
another on bit. Right shift 4 more times to get (0000 0101). Now we
see a 1 in the right-most bit again. This tells us the integers
are off by more than 1 bit.
