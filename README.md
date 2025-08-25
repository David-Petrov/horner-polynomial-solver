# Horner solver
A simple calculator for finding the rational roots of polynomials with integer coefficients using Horner's method.

## Usage
The input is space separated coefficients of the polynomial in ascending power of the variable. 

### Example:
```
Enter space separated coefficients:
6 -7 0 1
Rational roots of the polynomial 6 - 7x^1 + x^3 are:
1, -3, 2
```

## How it works
This calculator employs [the Rational Root Theorem](https://en.wikipedia.org/wiki/Rational_root_theorem) for electing all possible rational candidates for a root. Then, it evaluates the polynomial for all candidates using [Horner's scheme](https://en.wikipedia.org/wiki/Horner%27s_method) (essentially `n` additions and `n` multiplications for a `n`-degree polynomial). If at any point the polynomial evaluates to 0, then we've found a rational root. The theorem guarantees that all rational roots will be found this way.
