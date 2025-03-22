### Digit-Power compression

Take a number, say `7290125`. It is stored as an unsigned 64-bit integer. We want to represent this number as a byte-pair sequence.

The idea of this algorithm is to use a pair of bytes to calculate part of the digits.

In the byte-pair, `(a, b)`, both a and b can have values from `0` to `255`. The idea is to translate the byte-pair as an exponent expression. Basically part of the digits of the original number is the same as the digits in `a ^ b`

In the above number, `729 = 27 ^ 2` and `125 = 5 ^ 3`. But what about the `0` in the middle? Well, if there are `0` values in the middle that have not been encoded yet, then we use the byte pair `(0, n)`. Basically it means the digit `0` is repeaing `n` times.

So the result is `7290125 = (27, 2) (0, 1) (5, 3)`

Remember that this is a byte-pair sequence, so the 64-bit unsigned integer has been represented as 3 pairs of bytes, so a 75% reduction in size in the case of `7290125`.

This simple explanation is theoretical. Not all numbers can result in reduction like this. This program calculates the compression factor for a range of numbers. For numbers above a certain threshold, there is no compression at all, in fact there could be an increase in size.

