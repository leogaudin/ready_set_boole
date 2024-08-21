**`ready_set_boole` is a 42 project introducing the basics of boolean algebra**.

## Table of Contents

- [Exercises](#exercises) 🏋🏻
	- [00 - Adder](#00---adder) ➕
	- [01 - Multiplier](#01---multiplier) ✖️
- [Resources](#resources) 📖

# Exercises

## 00 - Adder

> You must write a function that takes as parameters two natural numbers a and b and returns one natural number that equals a + b. However the only operations you’re allowed to use are:
>
> - `&` (bitwise AND)
> - `|` (bitwise OR)
> - `^` (bitwise XOR)
> - `<<` (bitwise left shift)
> - `>>` (bitwise right shift)
> - `=` (assignment)
> - `==`, `!=`, `<`, `>`, `<=`, `>=` (comparison)

If we really simplify this exercise, it basically asks us to add two numbers in **base 10**, using only operations from **base 2**.

### Bases

A base means how important digits in a number are.

In **base 10**, the rightmost digit is the unit, the next digit is the tens, the next is the hundreds, etc.

$$
\begin{array}{c}
  1 & 0 & 1 & 1 \\
  \\
  10^3 & 10^2 & 10^1 & 10^0 \\
  1000 & 100 & 10 & 1 \\
\end{array}
$$

This number is equal to:

$$
(1 * 10^3) + (0 * 10^2) + (1 * 10^1) + (1 * 10^0)\\
= 1000 + 0 + 10 + 1\\
= 1011
$$

> Given that we use base 10 in our daily lives, we are used to it and don't even think about the previous calculation. But what if we used another base?

In **base 2**, the rightmost digit is the unit, the next is the twos, the next is the fours, etc.

$$
\begin{array}{c}
  1 & 0 & 1 & 1 \\
  \\
  2^3 & 2^2 & 2^1 & 2^0 \\
  8 & 4 & 2 & 1 \\
\end{array}
$$

If we perform the same calculation as before, we get:

$$
(1 * 2^3) + (0 * 2^2) + (1 * 2^1) + (1 * 2^0)\\
= 8 + 0 + 2 + 1\\
= 11
$$

> Base 2 is also known as binary, and it's the base used by computers.

### How to perform an addition

It may sound weird, but to really nail this exercise, we need to review how to add two numbers in base 10.

Let's take the following example:

$$
\begin{array}{r}
  50 \\
\+ 65 \\
\hline
  115 \\
\end{array}
$$

For each column:
1. We sum the digits
2. If needed, carry the excess to the next column (adding a little "1" on the left)

In our example:

$$
\begin{array}{r}
  ^1	& & \\
  & 5 & 0 \\
\+ & 6 & 5 \\
\hline
  1 & 1 & 5 \\
\end{array}
$$

---

In base 2, the same principle applies, but we have fewer digits to work with.

Let's take the following example of $5 + 6 = 11$ in base 2:

$$
\begin{array}{r}
  101 \\
\+ 110 \\
\hline
  1011 \\
\end{array}
$$

For each column:
1. We sum the digits
2. If needed, carry the excess to the next column (adding a little "1" on the left)

In our example:

$$
\begin{array}{r}
  ^1	& & \\
  & 1 & 0 & 1 \\
\+ & 1 & 1 & 0 \\
\hline
  1 & 0 & 1 & 1 \\
\end{array}
$$

Do you see the pattern?
- The combinations of a $1$ and a $0$ give a $1$.
- The combination of two $1$ gives a $0$ and carries a $1$ to the next column.

### Bitwise operations

If you already know about bitwise operations, you might have noticed that the previous examples are respectively the results of:
- a bitwise XOR
- a bitwise AND followed by a left shift

#### XOR

The XOR operation is a bitwise operation that returns $1$ if the bits are different, and $0$ if they are the same.

$$
\begin{array}{c|c|c}
  A & B & A \oplus B \\
  \hline
  0 & 0 & 0 \\
  0 & 1 & 1 \\
  1 & 0 & 1 \\
  1 & 1 & 0 \\
\end{array}
$$

If you XOR the numbers $5$ and $6$, you get:

$$
\begin{array}{c|c}
  5 & 101 \\
  6 & 110 \\
  5 \oplus 6 & 011 \\
\end{array}
$$

#### AND

The AND operation is a bitwise operation that returns $1$ if both bits are $1$, and $0$ otherwise.

$$
\begin{array}{c|c|c}
  A & B & A \\& B \\
  \hline
  0 & 0 & 0 \\
  0 & 1 & 0 \\
  1 & 0 & 0 \\
  1 & 1 & 1 \\
\end{array}
$$

If you AND the numbers $5$ and $6$, you get:

$$
\begin{array}{c|c}
  5 & 101 \\
  6 & 110 \\
  5 \\& 6 & 100 \\
\end{array}
$$

#### Left shift

The left shift operation moves the bits to the left by a certain number of positions, adding $0$ on the right.

For example, if we left shift $5$ (which is $101$ in base 2) by $2$, we get $10100$.

$$
\begin{array}{c|c}
  5 & 101 \\
  5 << 2 & 10100 \\
\end{array}
$$

### Solution

Let's take once again the example of summing $5$ and $6$, in base 2:

$$
\begin{array}{r}
  ^1	& & \\
  & 1 & 0 & 1 \\
\+ & 1 & 1 & 0 \\
\hline
  1 & 0 & 1 & 1 \\
\end{array}
$$

If we XOR them, we get:

$$
\begin{array}{r}
  & 1 & 0 & 1 \\
\oplus & 1 & 1 & 0 \\
\hline
  0 & 0 & 1 & 1 \\
\end{array}
$$

If we AND them and left shift the result, we get:

$$
\begin{array}{r}
  & 1 & 0 & 1 \\
\\& & 1 & 1 & 0 \\
\hline
  & 1 & 0 & 0 \\
\hline
  1 & 0 & 0 & 0 \\
\end{array}
$$

Finally, XORing the result with the carry gives us the final result:

$$
\begin{array}{r}
& & 1 & 0 & 1 \\
&\oplus & 1 & 1 & 0 \\
\hline
  & 0 & 0 & 1 & 1 \\
\oplus & 1 & 0 & 0 & 0 \\
\hline
  &1 & 0 & 1 & 1 \\
\end{array}
$$

> If you try to implement this solution, you will notice that it only works for one carry.
>
> Good luck implementing it for multiple carries!

# Resources

- [📺 Add Two Numbers Without The "+" Sign (Bit Shifting Basics)](https://www.youtube.com/watch?v=qq64FrA2UXQ)
