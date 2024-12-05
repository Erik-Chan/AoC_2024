# Day 2 - Advent of Code 2024

This directory contains the solution for day 2 of the Advent of Code 2024.

## Problem Description
The given input is an array of non-negative integers. We say that a list is "safe" if the following criteria are met:

1. The list is an increasing or decreasing sequence.
2. Two consecutive integers differ by at least one or at most three.

The goal is to determine how many arrays provided are safe.


For the full problem description, visit the [Advent of Code Day 2 page](https://adventofcode.com/2024/day/2).

## Input Data
The input data can be found in input.txt.

## Solution

Given a list $\ell = a_0, a_1, \dots, a_n$, we define the list as "safe" if the following conditions hold for all $i \in \{0, 1, \dots, n-1\}$:

1. $a_i \leq a_{i+1}$ or $a_i \geq a_{i+1},$

2. $1 \leq \lvert a_i - a_{i+1} \rvert \leq 3.$

To solve this, we will use two pointers to traverse the vector, keeping track of the current element (`curr`) and the previous element (`prev`).

### Checking for increasing or decreasing sequence

We start by initializing two boolean flags, `increasing` and `decreasing`, both set to `true`. These flags will help us determine if the sequence is increasing or decreasing.

At each step in our loop, we compare `prev` and `curr`:
- If `prev > curr` (indicating a possible decreasing sequence), we set the `increasing` flag to `false`.
- Conversely, if `prev < curr`, we set the `decreasing` flag to `false`.

If at any point both `increasing` and `decreasing` are `false`, we can immediately terminate the loop and determine that the sequence is "unsafe."

Note that the problem allows for non-strictly increasing or decreasing sequences. For instance, the constant sequence $[1, 1, 1, 1]$ is both increasing and decreasing. However, in our implementation, we focus on strictly increasing or decreasing sequences, as the second condition would cause non-strict sequences to be marked as "unsafe."

#### Validating Adjacent Differences

In each iteration, we also check if the absolute difference between adjacent elements satisfies $1 \leq \lvert a_i - a_{i+1} \rvert \leq 3$. Given that we are checking for strictly increasing or decreasing sequences in the previous step, this ensures the condition $1 \leq \lvert a_i - a_{i+1} \rvert$ is already met.

If any adjacent difference exceeds 3, we terminate the loop early and conclude the sequence is "unsafe."

## Running the Solution

To run the solution for Day 2, navigate to this directory and execute the following commands:

```sh
cargo build
cargo run
