**`ready_set_boole` is a 42 project introducing the basics of boolean algebra**.

> **⚠️ This guide assumes you are already an experienced programmer, and are familiar with classic data structures (i.e. stacks and binary trees).**

## Table of Contents

- [Exercises](#exercises) 🏋🏻
	- [00 - Adder](#00---adder) ➕
	- [01 - Multiplier](#01---multiplier) ✖️
	- [02 - Gray Code](#02---gray-code) 🔢
	- [03 - Boolean Evaluation](#03---boolean-evaluation) 🧮
	- [04 - Truth Table](#04---truth-table) 📊
  - [05 - Negation Normal Form](#05---negation-normal-form) ➖
  - [06 - Conjunctive Normal Form](#06---conjunctive-normal-form) ⋀
  - [07 - SAT](#07---sat) 🧩
  - [08 - Powerset](#08---powerset) 🔋
  - [09 - Set Evaluation](#09---set-evaluation) 🧾
- [Resources](#resources) 📖

# Exercises

## 00 - Adder

```rust
fn adder(a: u32, b: u32) -> u32;
```

> You must write a function that takes as parameters two natural numbers a and b and returns one natural number that equals a + b. However the only operations you’re allowed to use are:
>
> - `&` (bitwise AND)
> - `|` (bitwise OR)
> - `^` (bitwise XOR)
> - `<<` (bitwise left shift)
> - `>>` (bitwise right shift)
> - `=` (assignment)
> - `==`, `!=`, `<`, `>`, `<=`, `>=` (comparison)

---

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

```rust
fn multiplier(a: u32, b: u32) -> u32;
```

> You must write a function that takes as parameters two natural numbers a and b and returns one natural number that equals a * b. However the only operations you’re allowed to use are:
>
> - `&` (bitwise AND)
> - `|` (bitwise OR)
> - `^` (bitwise XOR)
> - `<<` (bitwise left shift)
> - `>>` (bitwise right shift)
> - `=` (assignment)
> - `==`, `!=`, `<`, `>`, `<=`, `>=` (comparison)

---

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

To be clearer, let's rewrite the operation as:

$$
6 * 53 \\
= (6 * 3) * 10^0 + (6 * 5) * 10^1 \\
= 6 * 3  + 6 * 50
$$

**To sum things up, we multiply the multiplicand by each digit of the multiplier, shifting appropriately the result to the left.**

> e.g. $6 * 5$, shifted by $1$, so $6* 50$

Now, we need to do the same thing, in base 2.

Unlike base 10, we only have two digits: $0$ and $1$. So we only have two scenarios when going through the digits of the multiplier:
- If the digit is $0$, we add $0 * \text{multiplicand}$ to the result → **nothing to do**.
- If the digit is $1$, **we add $1 * \text{multiplicand}$ to the result**.

#### Example

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
\+ & 1& 1& 0 & 1 &  &  & & \color{red}(1101 \times 1 ~\text{shifted thrice})
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
> $4294967296 / 1024 = 4194304$, literally 4 million times faster.

## 02 - Gray Code

```rust
fn gray_code(n: u32) -> u32;
```

> You must write a function that takes an integer `n` and returns its equivalent in Gray code.

---

This exercise is simpler than the others in the sense that **it does not require us to convert decimal problems to Boolean algebra**.

The Gray Code is a variant of binary that was invented to facilitate transmissions, by making each consecutive value different from its previous one by only one bit.

> It's a bit confusing, but for example, $4$ in Gray code is written $0110$, and $5$ is written $0111$.
>
> Here for instance, only the last bit has changed. That is the concept of Gray code.

The purpose of Gray code and its uses are more complex, but what really matters to us here is: **how do we translate normal binary to Gray code?**

Well, it is actually quite simple now that we are comfortable with Boolean algebra.

As said before, its recipe is just a set of bitwise operations:

1. We initialize a number (our to-be Gray number) to 0
3. We copy the most significant bit (the one all the way left) to this number
4. We then append to the right of this number, the result of XOR on each bit and its neighbor to the right

### Example

To make it clearer, let's take the example of $13$, in binary $1101$.

1. We initialize our Gray number to $0$.
2. We copy the most significant bit (**1** 101) to our Gray number, so it becomes $1$.
3. We XOR the first bit with the second bit: **11** 01.
    - $1 \oplus 1 = 0$, so we append $0$ to our Gray number.
    - **It becomes $10$**.
4. We XOR the second bit with the third bit: 1 **10** 1.
    - $1 \oplus 0 = 1$, so we append $1$ to our Gray number.
    - **It becomes $101$**.
5. We XOR the third bit with the fourth bit: 11 **01**.
    - $0 \oplus 1 = 1$, so we append $1$ to our Gray number.
    - **It becomes $1011$**.

And that's it! We're done converting $13$ in Gray code, which gives $1011$ in binary → $11$ in decimal.

> 💡 We are working with 32-bit integers, so you can either perform those operations 32 times, or determine the bit length $x$ and perform those operations $x$ times.
>
> We then get a complexity of $O(x)$, where $x$ cannot exceed $32$, and will most likely be much smaller (e.g. $O(4)$ for converting $13$).

## 03 - Boolean Evaluation

```rust
fn eval_formula(formula: &str) -> bool;
```

> You must write a function that takes as input a string that contains a propositional formula in reverse polish notation, evaluates this formula, then returns the result.

---

This one is actually pretty simple.

Reverse Polish Notation (RPN) is a way to write mathematical expressions without parentheses.

> For example, the expression $(3 + 4) * 5$ can be written in RPN as `3 4 + 5 *`.

It was already used in the Module 09 of the C++ modules, so you might be familiar with it.

You basically push any operand you encounter to a stack, and when you encounter an operator, you pop the last two operands, apply the operator, and push the result back to the stack.

Here is a simplified version of the algorithm:

```rust
for c in chars {
	if c == operand {
		stack.push(operand);
	}
	else {
		let b = stack.pop();
		let a = stack.pop();

		match c {
			operator => stack.push(a operator b),
			some_other_operator => stack.push(a some_other_operator b),
			_ => {
				println!("Invalid operator");
				return false;
			}
		}
	}
}
```

> 💡 You can use a `Vec` as a stack, and use the `pop` and `push` methods to manipulate it, it's exactly the same.

Now we just need to adapt this algorithm to what the exercise asks:

```rust
match c {
	'&' => stack.push(a && b),
	'|' => stack.push(a || b),
	'^' => stack.push(a ^ b),
	'>' => stack.push(!a || b),
	'=' => stack.push(a == b),
	_ => {
		println!("Invalid operator");
		return false;
	}
}
```

> Do not forget to handle the `!` operator, which is unary.

And that's it! You have your solution.

> ⚠️ From exercise 05 onwards, we will start to work with more complex concepts, such as **Negation Normal Form** and **Conjunctive Normal Form**.
>
> For those purposes, it might be good implementing a **Binary Tree** to represent the formula.
>
> The logic is the same as with the stack, but there is a lot of initial code to write to set up the tree.

## 04 - Truth Table

```rust
fn print_truth_table(formula: &str);
```

> You must write a function that takes as input a string that contains a propositional formula in reverse polish notation, and writes its truth table on the standard output.
>
> A formula can have up to 26 distinct variables (A...Z), one per letter. Each variable can be used several times.

---

This exercise is a bit more complex than the previous one, but it does not really require any new concepts.

We will not cover everything here, but we will give you a hint on how to proceed.

### Steps

1. **Determine the variables** in the formula.
> 💡 To the accepted characters from the previous exercise, simply add those between the ASCII `A` and `Z`, and push any of them to a vector (watch out for duplicates).
2. **Generate all possible sets of values** for the variables.
> 💡 We will cover this later, but basically looking at all possible boolean combinations.
>
> This set of combination has a size of $2^n$, where $n$ is the number of variables.
3. **Evaluate the formula** for each set of values.
> 💡 You can use the previous exercise to evaluate the formula.
>
> Simply replace the variables by their values in the set, and evaluate the formula.
>
> For example, if you have the set `[true, false, true]`, replace `A` by `1`, `B` by `0`, and `C` by `1`, and call `eval_formula`.

### Generating all possible sets of values

To generate all possible sets of values for the variables, you can use a recursive function.

Here is a simplified version of the algorithm:

```rust
fn generate_sets(variables: Vec<char>, set: Vec<bool>, index: usize) {
	if index == variables.len() {
		// Evaluate the formula with the set
		return;
	}

	set[index] = true;
	generate_sets(variables, set, index + 1);

	set[index] = false;
	generate_sets(variables, set, index + 1);
}
```

## 05 - Negation Normal Form

```rust
fn negation_normal_form(formula: &str) -> String;
```

> You must write a function that takes as input a string that contains a propositional formula in reverse polish notation, and returns an equivalent formula in **Negation Normal Form** (NNF), **meaning that every negation operators must be located right after a variable**.
>
> The result must only contain variables and the following symbols: `!`, `&` and `|` (even if the input contains other operations).

---

> ⚠️ As said before, we will start to work with a **Binary Tree** to represent the formula.
>
> Every node of the tree will be an operator, and every leaf will be a variable.
>
> Example:
>
> ![Boolean Binary Tree](https://assets.leetcode.com/uploads/2022/05/16/example1drawio1.png)

### Rewrite rules

To complete this exercise, we will need to apply some "rewrite rules" to the tree, provided by the subject.

1. **Elimination of double negation**

$$
\neg \neg A \rightarrow A
$$

2. **Material conditions**

$$
A \rightarrow B \rightarrow \neg A \vee B
$$

3. **Equivalence**

$$
A \leftrightarrow B \rightarrow (A \rightarrow B) \wedge (B \rightarrow A)
$$

4. **De Morgan's laws**

$$
\neg (A \wedge B) \rightarrow \neg A \vee \neg B
$$

$$
\neg (A \vee B) \rightarrow \neg A \wedge \neg B
$$

<!-- 5. **Distributivity**

$$
A \wedge (B \vee C) \rightarrow (A \wedge B) \vee (A \wedge C)
$$

$$
A \vee (B \wedge C) \rightarrow (A \vee B) \wedge (A \vee C)
$$ -->

### Steps

1. **Parse the formula** to build the binary tree.
2. **Apply all the mentioned rewrite rules** to the tree.
> ⚠️ XOR is not mentioned in the rewrite rules, you have to figure out how to rewrite it!
3. **Rebuild the formula** from the tree.

To develop step 2 a bit more, here is how you can apply some rewrite rules:

- **Material Conditions**: Every node `>` with children `A` and `B`, will need to become a node `|` with children `!A` and `B`.
- **De Morgan's laws**: Every node `!` with a child `&` with children `A` and `B`, will need to become a node `|` with children `!A` and `!B`.

> 💡 Use a recursive function to apply the rewrite rules to the tree.

## 06 - Conjunctive Normal Form

```rust
fn conjunctive_normal_form(formula: &str) -> String;
```

> You must write a function that takes as input a string that contains a propositional formula in reverse polish notation, and returns an equivalent formula in **Conjunctive Normal Form** (CNF). This means that in the output, every negation must be located right after a variable and **every conjunction must be located at the end of the formula**.

---

This sounds very similar to the previous exercise, but it is actually a bit more complex.

Yes, you first need to convert the formula to NNF, but then you need to apply a few more rules to convert it to CNF.

### The last rewrite rule

The last rewrite rule is the **Distributivity**.

$$
A \wedge (B \vee C) \rightarrow (A \wedge B) \vee (A \wedge C)
$$

$$
A \vee (B \wedge C) \rightarrow (A \vee B) \wedge (A \vee C)
$$

Distibutivity basically allows us to "distribute" a conjunction over a disjunction, and vice versa.

> 🤓 More simply put, it means that if you have a `|` above a `&`, distributivity allows to "switch" them.
>
> That is good news, given that we want to bring all the `&` to the end of the formula (i.e. at the upper level of the tree).

### Steps

1. **Apply NNF** to the formula.
2. **Build the binary tree**.
3. **Apply the first distributivity rule** to the tree.
4. **Rebuild the formula** from the tree.

This being done, you might have an output that looks like this:

```
Formula: AB&CD&|
CNF: AC|AD|&BC|BD|&&
```

If we were to convert this CNF back to a tree, it would look like `(A | C) & (A | D) & (B | C) & (B | D)`.

So the objective is achieved, all the `&` are at the top-level!

However, the RPN notation is not respected, some `&` are not at the very end of the formula.

**The final step is simply to move all the `&` to the end of the formula** (yes, manipulating the RPN string directly).

> ⚠️ This can be done safely only because all the `&` are at the top-level. If you were to manipulate an RPN notation without ensuring this, you would most likely break the formula.

## 07 - SAT

```rust
fn sat(formula: &str) -> bool;
```

> You must write a function that takes as input a string that contains a propositional formula in reverse polish notation and tells whether it is satisfiable.
>
> The function has to determine if there is at least one combination of values for each variable of the given formula that makes the result be ⊤. If such a combination exists, the function returns `true`, otherwise, it returns `false`.

---

This exercise may sound complex, but you already have all the tools to solve it since exercise 04.

Remember the truth table? It was used to evaluate all possible combinations of values for the variables.

Well, this is the only thing you need to do here.

1. **Generate all possible sets of values** for the variables.
2. **Evaluate the formula** for each set of values.
3. If the formula evaluates to `true` for at least one set of values, return `true`.
4. Return `false` otherwise.

## 08 - Powerset

```rust
fn powerset(set: Vec<i32>) -> Vec<Vec<i32>>;
```

> You must write a function that takes as input a set of integers, and returns its powerset.

---

The powerset of a set is the set of all its subsets.

> That doesn't make it any clearer.

Let's take the set `[1, 2, 3]`.

The powerset of this set is:

```
[
  [], // The empty set
  [1], [2], [3], // All the singletons
  [1, 2], [1, 3], [2, 3], // All the pairs
  [1, 2, 3] // The set itself
]
```

So, a powerset simply contains all the possible combinations of elements of the set.

### Implementation

To achieve this, we can use the special property of binary numbers.

For a set of size $n$, the powerset will have $2^n$ elements.

Moreover, going from $0$ to $2^n - 1$ in binary will give us all the possible combinations of `0` and `1` for a set of size $n$.

> Example: for the set `[1, 2, 3]` of length $3$, we will go from $0$ to $2^3 - 1 = 7$ in binary, and push the values of the set where the bit is `1`.
>
> ```
> // 123
>
> 0: 000 → []
> 1: 001 → [3]
> 2: 010 → [2]
> 3: 011 → [2, 3]
> 4: 100 → [1]
> 5: 101 → [1, 3]
> 6: 110 → [1, 2]
> 7: 111 → [1, 2, 3]
> ```
>

And that's it! We have our powerset.

## 09 - Set Evaluation

```rust
fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32>;
```

> You must write a function that takes as input a string that contains a propositional formula in reverse polish notation, and a list of sets (each containing numbers), then evaluates this list and returns the resulting set.

---

Once again, the subject is not very clear, but it is actually quite simple.

### Link between Logic and Set Theory

> This is extracted from the subject, and will help you understand the exercise.

The following logical operations have a counterpart in set theory (they are not the only operations that exist though):

<div align='center'>

Boolean Algebra | Boolean Algebra name | Set Theory | Set Theory name
--- | --- | --- | --- | ---
$\neg A$ | Negation (NOT) | $A^c$ | Complement
$A \vee B$ | Disjunction (OR) | $A \cup B$ | Union
$A \wedge B$ | Conjunction (AND) | $A \cap B$ | Intersection

</div>

> In programming, the symbols for these operations are respectively `!`, `|`, and `&`.

#### Negation

Let's take the set `[1, 2, 3]`.

The complement of this set is the set of all the numbers that are not in the set.

If we are working with the universal set of all integers, the complement of `[1, 2, 3]` is $[-\infty, 0] \cup [4, +\infty]$.

> **Any number that is not in the set** is in the complement.
>
> So technically, the complement of the set is infinite, but the subject specifies that ***the globally encompassing set is considered to be the union of all the sets given as parameters***."
>
>

#### Disjunction

Let's take the sets `[1, 2, 3]` and `[4, 5, 6]`.

The **union** of these two sets is `[1, 2, 3, 4, 5, 6]`.

> **Any number that is in at least one of the sets** is in the union.

#### Conjunction

Let's take the sets `[1, 2, 3]` and `[2, 3, 4]`.

The **intersection** of these two sets is `[2, 3]`.

> **Any number that is in both sets** is in the intersection.

### Implementation

Now that we demystified those three operations, we know that our function will work exactly like the boolean evaluation function, but with different behaviors for `!`, `|`, and `&`.

1. **Convert the formula to CNF**.
2. **Parse the formula** to build the binary tree.
3. **Evaluate the formula** for each set of values.
4. **Apply the corresponding set operation** to the sets.

# Resources

- [📺 Add Two Numbers Without The "+" Sign (Bit Shifting Basics)](https://www.youtube.com/watch?v=qq64FrA2UXQ)
- [📺 Binary Multiplication](https://www.youtube.com/watch?v=PjmWG_8b3os)
---
- [💬 How can I perform multiplication, using bitwise operators?](https://stackoverflow.com/a/3722053/18370307)
- [💬 Grade School Multiplication Algorithm for Binary Numbers explanation](https://math.stackexchange.com/a/1118159)
---
- [📺 How To Convert Gray Code to Binary and Binary to Gray Code](https://www.youtube.com/watch?v=cbmh1DPPQyI)
---
- [📖 Implementing a Binary Tree in Rust for fun](https://rusty-ferris.pages.dev/blog/binary-tree-sum-of-values/)
---
- [💬 how to convert a propositional logical tree into conjunction normal form (CNF) tree](https://stackoverflow.com/a/16091759/18370307)
---
- [📺 Power Set](https://www.youtube.com/watch?v=FOQn8afAvLE)
- [📺 Generate Power set for a given set](https://www.youtube.com/watch?v=8xQXq5JLhEY) (bitwise way)
