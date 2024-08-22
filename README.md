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

## 01 - Multiplier

> You must write a function that takes as parameters two natural numbers a and b and returns one natural number that equals a * b. However the only operations you’re allowed to use are:
>
> - `&` (bitwise AND)
> - `|` (bitwise OR)
> - `^` (bitwise XOR)
> - `<<` (bitwise left shift)
> - `>>` (bitwise right shift)
> - `=` (assignment)
> - `==`, `!=`, `<`, `>`, `<=`, `>=` (comparison)

### Quick and dirty solution

If you are not familiar with bitwise operations, you might be tempted to use a loop to add `a` to a result `b` times.

```rust
pub fn multiplier(a: u32, b: u32) -> u32 {
	let mut result = 0;

	for _ in 0..b {
		result = adder(result, a);
	}

	return result;
}
```

This works. Literally. You can submit it.

However, it has a complexity of $O(b)$.

> i.e. it takes $b$ iterations to complete.
>
> We are working with `u32`, so the worst case scenario is $O(2^{32}) = O(4 294 967 296)$.

### Bitwise solution

There is a much faster way to multiply two numbers using bitwise operations.

> Up to 4 million times faster, to be exact.

Let's take the example of multiplying $53$ by $6$:

$$
\begin{array}{r}
		&&&& 1 & 1 & 0 & 1 & 0 & 1 \\
\times	&&&&&&		& 1 & 1 & 0 \\
\hline
		& 1 & 0 & 0 & 1 & 1 & 1 & 1 & 1 & 0 \\
\end{array}
$$

If we look at it, we don't really see any pattern like in the previous exercise.

However, putting the base 10 multiplication "on paper" as we did before, we can see that it's just a series of additions.

$$
6 * 53 = (6 * 3) + (6 * 50)
$$

$$
\begin{array}{r}
		&& 5 & 3 \\
\times	&&& 6 \\
\hline
		&& 1& 8 \\
\+		& 3 & 0 & 0 \\
\hline
		& 3 & 1 & 8 \\
\end{array}
$$

$6 * 3$ equals $18$, and $6 * 50$ equals $300$.

Rather, we can note it as $6 * 5$, adapted to the position of the $5$ (here $10^1$).

To be clearer, I'm going to rewrite the operation as:

$$
6 * 53 \\
= (6 * 3) * 10^0 + (6 * 5) * 10^1 \\
= 6 * 3  + 6 * 50
$$

**To sum things up, we multiply the multiplicand by each digit of the multiplier, shifting appropriately the result to the left.**

> e.g. $6 * 5$, shifted by $1$, so $6* 50$

---

Now, we need to do the same thing, in base 2.

Unlike base 10, we only have two digits: $0$ and $1$. So we only have two scenarios when going through the digits of the multiplier:
- If the digit is $0$, we add $0 * \text{multiplicand}$ to the result → **nothing to do**.
- If the digit is $1$, **we add $1 * \text{multiplicand}$ to the result**.

Let's take the example of multiplying $13$ with $11$:

$$
\begin{matrix}
& & & & & 1&1&0&1 \\
& & & &\times & \color{red}{\rm 1} & \color{purple}{\rm 0} & \color{blue}{\rm 1} & \color{green}{\rm 1}
\end{matrix}
$$

If we break it down in partial products as we just saw:

$$
\begin{matrix} & & & & 1 & 1 & 0 & 1 & \color{green}(1101 \times 1) \\
~ & & & 1 & 1 & 0 & 1 & & \color{blue}(1101 \times 1 ~\text{shifted once})\\
~ & & 0 & 0 & 0 & 0 &  & & \color{purple}(1101 \times 0 ~\text{shifted twice})\\
+ & 1& 1& 0 & 1 &  &  & & \color{red}(1101 \times 1 ~\text{shifted thrice})
\end{matrix}
$$

In a more readable way:

$$1101 + 11010 + 0 + 1101000$$

> We added the implicit zeros that were absent in the previous example.

Which is equal in base 10 to:

$$13 + 26 + 0 + 104 = 143$$

All good! We can see that the result is indeed equal to $13 * 11$.

**To recap:**

- We iterate through the bits of `b`.
> We can do this by shifting `b` to the right by `i`.
- If the bit is $1$, we add `a` to the result, shifted by the position `i` of the bit.

> 💡 To check if the rightmost bit is 0 or 1, we can simply check if `rightmost_bit & 1 == 0`

We now have our clean solution!

> Note how we passed from a complexity of $O(b)$ to $O(n^2)$, where $n$ is the number of bits of `b`.
>
> We are working with `u32`, so the worst case scenario is $O(32^2) = O(1024)$.
>
> $4294967296 / 1024 = 4194304$, I did not lie when I said it was up to 4 million times faster.

# Resources

- [📺 Add Two Numbers Without The "+" Sign (Bit Shifting Basics)](https://www.youtube.com/watch?v=qq64FrA2UXQ)
- [📺 Binary Multiplication](https://www.youtube.com/watch?v=PjmWG_8b3os)
- [💬 How can I perform multiplication, using bitwise operators?](https://stackoverflow.com/a/3722053/18370307)
- [💬 Grade School Multiplication Algorithm for Binary Numbers explanation](https://math.stackexchange.com/a/1118159)
