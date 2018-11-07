# Scan Benchs

[![Build Status](https://travis-ci.org/vain0x/scan-bench.svg?branch=master)](https://travis-ci.org/vain0x/scan-bench)

## Input

```
a
b c
s
k
x1 x2 x3 ... xk
H W
#.#.#.#.#.#
.#.#.#.#.#.
#.#.#.#.#.#
N M
u1 v1 w1
u2 v2 w2
...
uM vM wM
```

### Input: Constraints

- 1 ≤ a ≤ 100
- 1 ≤ b, c ≤ 1000
- 1 ≤ |s| ≤ 100

- 1 ≤ K ≤ 100
- -10^9 ≤ xi ≤ 10^9 (for 1 ≤ i ≤ K)

- 1 ≤ H, W ≤ 100

- 1 ≤ N ≤ 10^5
- 1 ≤ M ≤ 2 * 10^5
- 1 ≤ ui, vi ≤ N (for 1 ≤ i ≤ M)
- 1 ≤ wi ≤ 10^9

Notes:

- First three lines are from [A - Welcome to AtCoder](https://beta.atcoder.jp/contests/practice/tasks/practice_1?lang=en).

## Codes

- 📁 c-scanf1
    - 🌐 C
    - `scanf` from `<stdio.h>`
- 📁 cin1
    - 🌐 C++
    - `std::cin` from `<iostream>`
        - ✔ `sync_with_stdio(false)` and `tie(nullptr)`
- 📁 rust-read1
    - 🌐 Rust
    - Original `read!` macro (slow)
        - ✘ Mutex lock per `read!` invocation
        - ✘ Temporary buffer per line
        - ✔ Short definition
- 📁 scan1
    - 🌐 Rust
    - Original `scan!` macro (fast)
        - ✔ Single mutex lock
        - ✔ Single temporary buffer
        - ✘ Verbose definition
