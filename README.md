Languages: C, Rust
## Overview
The Fibonacci sequence is one of the most extensively studied recurrence relations in mathematics and computer science. While often introduced as a simple numerical pattern, it plays a meaningful role in computational modeling, recursive algorithm analysis, and even fractal visualization research [1]. Because its mathematical definition is concise yet computationally non-trivial, Fibonacci serves as an effective benchmark for evaluating algorithmic efficiency.

Although the recurrence relation appears simple, different computational implementations produce dramatically different performance characteristics. In particular, the naive recursive implementation directly mirrors the mathematical definition but results in exponential growth due to repeated recomputation of subproblems [2]. In contrast, iterative and dynamic programming approaches eliminate redundant work and reduce runtime growth to linear time [3][4].
The sequence is formally defined by the recurrence:

$$
F(0) = 0
$$

$$
F(1) = 1
$$

$$
F(n) = F(n-1) + F(n-2), \quad n \ge 2
$$

A typical sequence up to $N = 10$ is shown below:

```text
0
1
1
2
3
5
8
13
21
34
55
```

To analyze performance formally, this report uses **Big-O notation**, which characterizes how runtime and memory requirements scale as input size increases [5] The theoretical complexity classifications for the implementations examined in this study are:
| Version               | Time Complexity | Space Complexity |
|-----------------------|----------------|------------------|
| Iterative             | $O(n)$         | $O(1)$           |
| Recursive (naive)     | $O(2^n)$       | $O(n)$           |
| Dynamic Programming   | $O(n)$         | $O(n)$           |

The **recursive** implementation exhibits exponential time complexity because each function call generates two additional calls (excluding base cases), producing a rapidly expanding computation tree [2].

The **iterative** implementation computes each Fibonacci value exactly once using a loop, requiring only constant auxiliary memory [3]. The dynamic programming implementation similarly avoids redundant computation by storing previously computed values, thereby maintaining linear time complexity at the cost of additional memory usage [4].

The **dynamic programming** implementation improves upon the recursive approach by eliminating repeated subproblem evaluation. Rather than recomputing `fib(n-2)` multiple times across different branches, previously computed results are stored in a data structure (such as an array or memoization table). Each subproblem is therefore solved exactly once, reducing the time complexity from exponential to linear [4].. However, this optimization introduces additional space usage, as intermediate Fibonacci values must be retained in memory. Consequently, dynamic programming maintains $O(n)$ time complexity while incurring $O(n)$ space complexity.

Beyond algorithm design, programming language choice may influence constant-factor performance differences. This report therefore evaluates each implementation in both C and Rust. C is a procedural language widely used in performance-critical systems programming due to its minimal abstraction and direct memory control [7]. Rust similarly targets systems programming but introduces compile-time memory safety guarantees through its ownership model [6] Prior benchmarking comparisons between C and Rust using Fibonacci workloads indicate that while minor constant-factor differences may appear, asymptotic growth behavior remains consistent across languages [8]

**The objectives of this study are:**

1. To empirically evaluate recursive, iterative, and dynamic programming Fibonacci implementations relative to theoretical Big-O expectations.

2. To analyze whether language-level differences between C and Rust meaningfully affect runtime behavior beyond constant factors.

3. By combining theoretical complexity analysis with empirical measurement, this report aims to demonstrate the dominant influence of algorithmic design on computational performance while contextualizing language-level differences within that framework. 


## Empirical Data & Discussion 
The empirical data was generated using the following files:

### C Implementation
- Core algorithm: [fib.c](c_impl/fib.c)
- Automation script: [c_runner.py](c_impl/c_runner.py)
- Operations dataset: [ops_c.csv](c_impl/ops_c.csv)
- Timing dataset: [timings_c.csv](c_impl/timings_c.csv)

The `c_runner.py` script was used to execute compiled C binaries across a range of input sizes and export results in CSV format.

### Rust Implementation
- Core algorithm: [main.rs](rust_impl/src/main.rs)
- Operations dataset: [ops_fib.csv](rust_impl/Final%20Data/ops_fib.csv)
- Timing dataset: [timings_fib.csv](rust_impl/Final%20Data/timings_fib.csv)

Rust execution timing and operation counting were built directly into the main program logic.

### Test Validation
- Output verification file: [test.csv](test.csv)

The `test.csv` file was used to verify correctness of computed Fibonacci outputs across implementations before collecting timing data.

---

## Operations Comparison

The number of operations for each algorithm was calculated by adding an addition counter directly inside the Fibonacci functions. While the exact count can vary slightly depending on where the counter is placed, it was inserted in the same logical position in both the C and Rust implementations, immediately at the addition operation responsible for computing the next Fibonacci value.

For the iterative and dynamic programming implementations, the counter increments once per addition inside the loop. For the recursive implementation, the counter increments each time the recursive call returns and performs the addition:


### Observed Behavior

- **Iterative Implementation**  
  Performs approximately (n − 1) additions.  
  Operations grow linearly with n.  
  This confirms **O(n)** time complexity.

- **Dynamic Programming Implementation**  
  Also performs approximately (n − 1) additions.  
  Each Fibonacci value is computed exactly once.  
  Operations grow linearly with n.  
  Time Complexity: **O(n)**  
  Space Complexity: **O(n)**

- **Recursive Implementation**  
  Generates repeated recomputation of subproblems.  
  Operation counts grow exponentially.

**The recurrence relation:** 

$T(n) = T(n - 1) + T(n - 2) + O(1)$

**Produces exponential growth:**

Time Complexity: **O(2ⁿ)**  
Space Complexity: **O(n)** (recursion stack depth)

**The exponential increase in operation counts confirms the recurrence tree analysis.**


This ensures the operation count reflects the true number of arithmetic additions performed by the algorithm.

Because the recursive implementation grows exponentially, only the first 28 values are shown below. Beyond n = 28, the recursive runtime increases rapidly and becomes impractical to execute within a reasonable time window.

---

### Operations Count (n ≤ 28)

| N | Iterative | Dynamic Programming | Recursive |
|---|-----------|--------------------|-----------|
| 1 | 1 | 0 | 0 |
| 2 | 3 | 1 | 1 |
| 3 | 6 | 3 | 4 |
| 4 | 10 | 6 | 11 |
| 5 | 15 | 10 | 26 |
| 6 | 21 | 15 | 57 |
| 7 | 28 | 21 | 120 |
| 8 | 36 | 28 | 247 |
| 9 | 45 | 36 | 502 |
| 10 | 55 | 45 | 1,013 |
| 11 | 66 | 55 | 2,036 |
| 12 | 78 | 66 | 4,083 |
| 13 | 91 | 78 | 8,178 |
| 14 | 105 | 91 | 16,369 |
| 15 | 120 | 105 | 32,752 |
| 16 | 136 | 120 | 65,519 |
| 17 | 153 | 136 | 131,054 |
| 18 | 171 | 153 | 262,125 |
| 19 | 190 | 171 | 524,268 |
| 20 | 210 | 190 | 1,048,555 |
| 21 | 231 | 210 | 2,097,130 |
| 22 | 253 | 231 | 4,194,281 |
| 23 | 276 | 253 | 8,388,584 |
| 24 | 300 | 276 | 16,777,191 |
| 25 | 325 | 300 | 33,554,406 |
| 26 | 351 | 325 | 67,108,837 |
| 27 | 378 | 351 | 134,217,700 |
| 28 | 406 | 378 | 268,435,427 |

---

### Interpretation of Operation Growth

- **Iterative Implementation**  
  The number of operations increases linearly with n.  
  This confirms time complexity of **O(n)** and space complexity of **O(1)**.

- **Dynamic Programming Implementation**  
  Operation growth is also linear.  
  Each Fibonacci value is computed exactly once.  
  This confirms time complexity of **O(n)** and space complexity of **O(n)**.

- **Recursive Implementation**  
  Operation counts approximately double each increment of n.  
  This demonstrates exponential growth consistent with **O(2^n)**.

The exponential increase in recursive operation counts directly confirms the recurrence tree analysis derived from:

$T(n) = T(n - 1) + T(n - 2) + O(1)$

Empirically, the operation counts match the theoretical complexity classification.

**For the full dataset, see:**
### C Operations Data
- [C Operations Data (ops_c.csv)](c_impl/ops_c.csv)

### Rust Operations Data
- [Rust Operations Data (ops_fib.csv)](Final%20Data/ops_fib.csv)
---

## Recursive Runtime (n ≤ 30)

### C Recursive Runtime

![C Recursive Runtime](images/c_recursive.png)

- [C Recursive Data (timings_c.csv)](c_impl/timings_c.csv)

### Rust Recursive Runtime

![Rust Recursive Runtime](images/rust_recursive.png)

[Rust Recursive Data (timings_fib.csv)](Final%20Data/timings_fib.csv)

Both languages exhibit clear exponential growth. Runtime increases slowly for small n, but grows sharply after approximately n ≈ 20. By n = 30, execution time increases dramatically.

---

## C and Rust Recursive Comparison

![C vs Rust Recursive Comparison](images/c_rust_recursive.png)

The curve shapes are nearly identical. Differences in runtime are constant-factor differences rather than asymptotic differences.

---
## Iterative and Dynamic Programming Runtime (n ≤ 5000)

Because both the iterative and dynamic programming implementations run in linear time, significantly larger input sizes were tested (n ≤ 5000). This allowed observation of long-term growth behavior beyond the small ranges used for recursive testing.

---

### Rust: Iterative vs Dynamic Programming

![Rust Iterative vs DP Runtime](images/rust_fib_itr_dp.png)

- **X-axis:** Input size `n`
- **Y-axis:** Execution time in seconds (Rust)

The Rust implementation shows steady linear growth for both algorithms. Minor spikes are visible but do not affect overall complexity classification.

The dynamic programming version is slightly slower than the iterative version due to:

- Additional memory allocation  
- Vector indexing  
- Increased memory writes  

---

### C: Iterative vs Dynamic Programming

![C Iterative vs DP Runtime](images/c_runtime_itr_dp.png)

- **X-axis:** Input size `n`
- **Y-axis:** Execution time in seconds (C)

The C implementation also demonstrates linear growth for both approaches. The dynamic programming version is marginally slower than the iterative version for the same reasons observed in Rust.

---

### Cross-Language Comparison (C vs Rust)

![C vs Rust Iterative and Dynamic Programming Runtime](images/fib_runtime_c_vs_rust.png)

- **X-axis:** Input size `n`
- **Left Y-axis:** Execution time in seconds (C)
- **Right Y-axis:** Execution time in seconds (Rust)

A dual-axis format was used because Rust runtimes were consistently slightly higher than C runtimes. Using a single axis would compress the C values and reduce readability.

#### Observed Behavior

Across both languages:

- Iterative implementation exhibits **O(n)** time complexity.
- Dynamic programming implementation exhibits **O(n)** time complexity.
- Dynamic programming is consistently slightly slower than iterative.
- C consistently executes faster than Rust.

However, these differences are **constant-factor differences**, not asymptotic differences. Both languages preserve identical growth trends.

---

### Speed Comparison Between Languages

| Version | Average Speed Difference | Max Speed Difference |
|----------|-------------------------|----------------------|
| Iterative | 1.603057459 | 8.032182 |
| Dynamic Programming | 3.39702299 | 14.377136 |
| Recursive | 3.942182071 | 54.427153 |

Differences become more noticeable at larger values of `n`, where runtime magnitude increases. Nevertheless, Big-O classification remains unchanged.

---
### Connection to Big-O Analysis

The empirical curves directly reflect the theoretical complexity derivations discussed earlier.

For the iterative and dynamic programming implementations, execution time increases proportionally with input size `n`. The nearly straight-line growth pattern observed in all linear-scale plots confirms **O(n)** time complexity. Each additional Fibonacci step requires one additional arithmetic operation, resulting in linear growth.

In contrast, the recursive implementation exhibits a rapidly accelerating curve. This aligns with the recurrence relation:

$$
T(n) = T(n - 1) + T(n - 2) + O(1)
$$

which expands into an exponential recursion tree. The steep upward curvature in the recursive runtime graph visually confirms **O(2^n)** growth.

Importantly, while C executes faster than Rust in absolute time, the slope of the curves remains unchanged between languages. This demonstrates that language choice affects constant factors, but does not alter asymptotic complexity.

---
### Complexity Summary

- Iterative: Time **O(n)**, Space **O(1)**
- Dynamic Programming: Time **O(n)**, Space **O(n)**
- Recursive: Time **O(2^n)**, Space **O(n)**

The empirical data strongly aligns with theoretical Big-O analysis.

---

### Measurement Variability and Limitations

Some runtime graphs show small spikes likely caused by:

- Operating system scheduling  
- Background processes  
- CPU frequency scaling  
- Cache effects  
- Timer precision limitations  

These fluctuations do not affect asymptotic growth classification.

Overall, algorithm design has a significantly greater impact on scalability than language choice.

---
## Testing Methodology and Runner Support

### Role of [test.csv](test.csv) in Validation

The `test.csv` file was used as an intermediate validation tool during development. Before generating the full empirical datasets (`timings_c.csv` and `ops_c.csv`), smaller controlled runs were written to `test.csv` to verify that the C executable produced correctly formatted comma-separated output. This allowed validation of parsing logic, timing precision formatting, and operation count alignment.

By using `test.csv` as a sanity-check dataset, potential formatting errors and indexing issues were resolved prior to large-scale automated execution. This ensured that the final exported CSV files used for visualization and analysis were structurally correct and consistent.
To ensure consistent and reproducible empirical results, both implementations used structured runner programs to automate execution and export CSV datasets.

### Rust Runner (Internal Automation with Timeout Control)

For the Rust implementation, a dedicated runner program was written using `std::process::Command` along with the `wait_timeout` crate. This allowed:

- Controlled execution of the compiled Fibonacci binary  
- Automatic timeout termination (60 seconds)  
- Parsing of comma-separated output  
- Separation of timing and operation data  
- Automatic CSV file generation  

The runner executed the compiled binary:

```rust
./target/release/fib_project
```
Below is the core execution logic used to invoke the binary with timeout protection:

```rust
use std::process::{Command, Stdio};
use std::time::Duration;
use std::fs::File;
use std::io::{Write, BufWriter};
use wait_timeout::ChildExt;

const TIMEOUT: u64 = 60;
const EXEC: &str = "./target/release/fib_project";

fn run_single(n: u32, typ: u32) -> Result<(Vec<String>, Vec<String>), String> {
    let mut child = Command::new(EXEC)
        .arg(n.to_string())
        .arg(typ.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    let timeout = Duration::from_secs(TIMEOUT);

    match child.wait_timeout(timeout).map_err(|e| e.to_string())? {
        Some(status) => {
            let output = child.wait_with_output().map_err(|e| e.to_string())?;
            if !status.success() {
                return Err(String::from_utf8_lossy(&output.stderr).to_string());
            }

            let line = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = line.trim().split(',').collect();

            let mut timings = Vec::new();
            let mut ops = Vec::new();

            for i in (0..parts.len()).step_by(2) {
                timings.push(parts[i].trim().to_string());
                ops.push(parts[i + 1].trim().to_string());
            }

            Ok((timings, ops))
        }
        None => {
            let _ = child.kill();
            Err("Timeout reached".to_string())
        }
    }
}
```

The `main()` function iterated from `n = 1` to `n = 5000`, writing results into:

-`timings_fib.csv`

-`ops_fib.csv`

This ensured consistent large-scale empirical measurement while safely handling exponential recursive growth.

### C Runner (Python-Based Automation)

For the C implementation, automation was handled externally using a Python runner script. The compiled C binary `(./fib)` was invoked using the `subprocess` module.

This approach allowed:

- Repeated automated execution  
- Parsing of comma-separated outputs  
- Formatting of timing precision  
- Export to CSV files  

The C binary prints:

_time,it_ops,dp_time,dp_ops,rec_time,rec_ops

The runner script parsed and separated these into timing and operation files:

```python
import csv
import subprocess

EXEC = "./fib"
N_MAX = 5000
ALG = "4"  # iterative + dp only

def run_one(n: int):
    out = subprocess.check_output([EXEC, str(n), ALG], text=True).strip()
    it_t, it_o, dp_t, dp_o, rec_t, rec_o = out.split(",")

    timings = [float(it_t), float(dp_t), float(rec_t)]
    ops = [int(it_o), int(dp_o), int(rec_o)]
    return timings, ops
```
Data was written to:

- [timings_c.csv](timings_c.csv)
- [ops_c.csv](ops_c.csv)


### Testing Strategy Summary

Both runners:

- Executed identical input ranges (`n ≤ 5000`)  
- Captured timing and operation counts separately  
- Exported structured CSV data  
- Enabled reproducible graph generation  

The Rust runner additionally implemented a 60-second timeout to prevent recursive execution from exceeding practical limits.

By using structured runners rather than manual execution, the empirical results:

- Are reproducible  
- Are consistent across languages  
- Reduce measurement noise  
- Provide large-scale data for visualization  

This testing infrastructure strengthens the validity of the empirical analysis and supports comparison between theoretical Big-O analysis and observed runtime behavior.

---

## Language Analysis
The implementations for this project were developed in both C and Rust to explore how language design influences performance, memory management, and implementation complexity.

The relevant files are:

### C Implementation
- [fib.c](c_impl/fib.c)
- [c_runner.py](c_impl/c_runner.py)
- [ops_c.csv](c_impl/ops_c.csv)
- [timings_c.csv](c_impl/timings_c.csv)

### Rust Implementation
- [main.rs](rust_impl/src/main.rs)
- [ops_fib.csv](rust_impl/Final%20Data/ops_fib.csv)
- [timings_fib.csv](rust_impl/Final%20Data/timings_fib.csv)

C was selected as the baseline systems language. Rust was selected as a modern systems language that emphasizes memory safety without sacrificing performance [6][7].

---

## Language 1: C

C provides direct memory control and minimal abstraction, making it ideal for performance analysis [7].

### Implementation Observations

#### Iterative in C

The iterative implementation required only two variables:

```c
unsigned long long a = 0ULL;
unsigned long long b = 1ULL;

for (unsigned long long i = 2ULL; i <= n; i++) {
    unsigned long long temp = a + b;
    ops++;
    a = b;
    b = temp;
}
```

#### Pseudocode (Iterative)

```text
function fib_iter(n):
    if n < 2:
        return n
    a <- 0
    b <- 1
    for i from 2 to n:
        temp <- a + b
        a <- b
        b <- temp
    return b
```

This approach avoids recursion entirely and does not require dynamic memory allocation. At each iteration, exactly one addition operation is performed.

Because each Fibonacci value is computed once and only two variables are maintained:

- **Time Complexity:** O(n)
- **Space Complexity:** O(1)

---

#### Dynamic Programming in C

The dynamic programming implementation required allocating an array to store computed values:

```c
unsigned long long *table =
    (unsigned long long *)malloc((n + 1ULL) * sizeof(unsigned long long));

table[0] = 0ULL;
table[1] = 1ULL;

for (unsigned long long i = 2ULL; i <= n; i++) {
    table[i] = table[i - 1ULL] + table[i - 2ULL];
    ops++;
}
```

#### Pseudocode (Dynamic Programming / Tabulation)

```text
function fib_dp(n):
    if n < 2:
        return n
    allocate table of size n+1
    table[0] <- 0
    table[1] <- 1
    for i from 2 to n:
        table[i] <- table[i-1] + table[i-2]
    return table[n]
```

This eliminates repeated subproblem computation.

- **Time Complexity:** O(n)
- **Space Complexity:** O(n)

In C, this required:
- Explicit array sizing
- Careful indexing
- Manual memory deallocation (`free`)
- Debugging potential segmentation faults

C does not perform automatic bounds checking. Incorrect indexing results in undefined behavior.

---

#### Recursive in C

The recursive implementation directly mirrors the mathematical definition:

$$
F(n) = F(n - 1) + F(n - 2)
$$

```c
static unsigned long long fib_rec_count(unsigned long long n,
                                        unsigned long long *ops) {
    if (n < 2ULL) {
        return n;
    }

    unsigned long long a = fib_rec_count(n - 1ULL, ops);
    unsigned long long b = fib_rec_count(n - 2ULL, ops);

    (*ops)++;
    return a + b;
}
```

#### Pseudocode (Recursive)

```text
function fib_recur(n):
    if n < 2:
        return n
    return fib_recur(n - 1) + fib_recur(n - 2)
```

The runtime follows the recurrence:

$$
T(n) = T(n - 1) + T(n - 2) + O(1)
$$

Which yields:

- **Time Complexity:** O(2^n)
- **Space Complexity:** O(n)

---

### Technical Considerations (C)

- 64-bit integer overflow occurs at n > 93
- No automatic memory safety
- No built-in bounds checking
- Manual heap allocation required for dynamic programming
- Slightly lower constant-factor runtime than Rust

C required greater attention to memory management but provided maximum control over execution behavior.

## Language 2: Rust
Rust was selected to compare a modern systems programming language that provides compile-time memory safety guarantees while maintaining performance characteristics similar to C [6]. Rust enforces strict ownership and borrowing rules, preventing invalid memory access at compile time. According to the official Rust documentation, its ownership model ensures memory safety without requiring a garbage collector [6].

The Rust implementation for this project is located here:
- Core algorithm + instrumentation: [main.rs](rust_impl/src/main.rs)
- Operations dataset: [ops_fib.csv](rust_impl/Final%20Data/ops_fib.csv)
- Timing dataset: [timings_fib.csv](rust_impl/Final%20Data/timings_fib.csv)

### Implementation Observations

#### Iterative in Rust

The iterative implementation closely mirrors the C version and uses constant auxiliary space by tracking only the two previous Fibonacci values.

Rust-style snippet (conceptual):
```rust
let mut a: u64 = 0;
let mut b: u64 = 1;

for _ in 2..=n {
    let temp = a + b;
    ops += 1;
    a = b;
    b = temp; 
}
```
### Pseudocode (Iterative):
```rust
function fib_iter(n):
    if n < 2:
        return n
    a <- 0
    b <- 1
    for i from 2 to n:
        temp <- a + b
        a <- b
        b <- temp
    return b
  ```
This version uses constant space and performs one addition per iteration. As expected, it exhibits:

Time Complexity: O(n) [2][5]

Space Complexity: O(1) [2][5]

### Dynamic Programming (Tabulation) in Rust

The dynamic programming version uses a `Vec<u64>` to store all computed Fibonacci values from `0..n`. The core allocation looks like:

```rust
let mut table = vec![0u64; (n + 1) as usize];
```

Using `Vec<u64>` provides:

- Automatic heap allocation  
- Built-in bounds checking  
- Prevention of memory corruption  
- Clear ownership semantics  

Rust required explicit type conversions when working with indices:

```rust
(n + 1) as usize
```

This reflects Rust’s strict type system, which prevents implicit casting errors and forces clarity.

#### Rust-Style Snippet (Conceptual)

```rust
let mut table = vec![0u64; (n + 1) as usize];
table[0] = 0;
table[1] = 1;

for i in 2..=n {
    let idx = i as usize;
    table[idx] = table[idx - 1] + table[idx - 2];
    ops += 1;
}
return table[n as usize];
```

#### Pseudocode (Dynamic Programming / Tabulation)

```text
function fib_dp(n):
    if n < 2:
        return n
    table[0] <- 0
    table[1] <- 1
    for i from 2 to n:
        table[i] <- table[i-1] + table[i-2]
    return table[n]
```

This eliminates repeated subproblems (each `F(i)` is computed once) and exhibits:

- **Time Complexity:** O(n) [4][5]  
- **Space Complexity:** O(n) [4][5]  

---

### Recursive in Rust

The recursive implementation matches the mathematical definition:

$$
F(n) = F(n-1) + F(n-2)
$$

#### Rust-Style Snippet (Conceptual)

```rust
fn fib_recur(n: u64) -> u64 {
    if n < 2 { return n; }
    fib_recur(n - 1) + fib_recur(n - 2)
}
```

#### Pseudocode (Recursive)

```text
function fib_recur(n):
    if n < 2:
        return n
    return fib_recur(n - 1) + fib_recur(n - 2)
```

The recursive implementation behaved identically in growth characteristics to the C version. However, Rust maintained memory safety guarantees throughout execution, eliminating undefined behavior that is possible in C [6].

This exponential growth follows directly from the recurrence tree expansion [2]. The runtime recurrence is:

$$
T(n) = T(n - 1) + T(n - 2) + O(1)
$$

Which yields:

- **Time Complexity:** O(2^n) [2][5]  
- **Space Complexity:** O(n) (call stack depth) [2][5]  

---

### Technical Considerations (Rust)

- Slight constant-factor runtime overhead compared to C  
- Strong compile-time memory safety guarantees  
- Reduced risk of segmentation faults  
- More verbose syntax due to explicit typing and ownership rules  

Rust sacrifices raw execution speed in exchange for safety guarantees and reliability [6].

## Conclusions / Reflection

In conclusion, **the iterative implementation consistently performed the fastest across both C and Rust**. This result aligns with the fact that the iterative version performs only one addition per loop iteration and uses constant auxiliary space. Because it avoids recursion and avoids allocating additional data structures, its runtime remained the most stable and predictable.

The dynamic programming implementation performed very similarly in practice. Although it has the same asymptotic time complexity of **O(n)**, it incurred slight overhead due to memory allocation and indexing operations. However, the difference between iterative and dynamic programming implementations was minor compared to the dramatic performance gap between those approaches and the recursive implementation.

The recursive implementation clearly demonstrated exponential growth. Even though it is the most mathematically elegant expression of the Fibonacci recurrence,

$$
F(n) = F(n - 1) + F(n - 2)
$$

its associated recurrence relation

$$
T(n) = T(n - 1) + T(n - 2) + O(1)
$$

leads to **O(2^n)** time complexity. Empirically, this made it impractical for moderate values of `n`. This experiment reinforced how quickly exponential growth becomes infeasible.

One of the most important lessons from this assignment was that algorithm selection dominates language choice. Switching from recursive to iterative reduced runtime exponentially. Switching from Rust to C produced only constant-factor improvements. This confirms that asymptotic complexity has far greater impact than language-level optimizations [5].

From a language perspective, C provided slightly faster execution and full control over memory. However, it required careful management and debugging discipline. Rust, while slightly slower in constant-factor terms, provided strong compile-time guarantees and prevented memory safety errors [6]. The ownership system initially required more deliberate thinking, but it eliminated entire classes of runtime bugs.

If this project were extended further, additional exploration could include:

- Implementing memoized recursion explicitly in C  
- Exploring iterative optimizations such as matrix exponentiation  
- Testing even larger input sizes using arbitrary precision integers  
- Investigating interoperability between C and Rust  

Overall, this assignment strengthened my understanding of recurrence relations, empirical performance validation, and the practical differences between systems programming languages. The most valuable takeaway was seeing theoretical Big-O analysis directly confirmed through measured runtime behavior.

## LLM Use Disclosure 
An LLM was used as a development assistant during this project. Assistance included:

- Structuring and refining runner scripts  
- Debugging execution and CSV parsing logic  
- Clarifying theoretical concepts related to recurrence relations and Big-O analysis 
- Helped format references for report, and helped find resources and articles

## References

[1] ScienceDirect. *Algorithmic analysis of recursive structures.*  
Accessed through: https://www.sciencedirect.com/science/article/abs/pii/S0960077924014036

[2] GeeksforGeeks. *Program for nth Fibonacci Number.*  
Accessed through: https://www.geeksforgeeks.org/dsa/program-for-nth-fibonacci-number/

[3] Stack Overflow. *An iterative algorithm for Fibonacci numbers.*  
Accessed through: https://stackoverflow.com/questions/15047116/an-iterative-algorithm-for-fibonacci-numbers

[4] Matthew Aquino. *Intro to Dynamic Programming with the Fibonacci Sequence.*  
Accessed through: https://matthewaquino.medium.com/intro-to-dynamic-programming-with-the-fibonacci-sequence-d9005e577854

[5] GeeksforGeeks. *Analysis of Algorithms | Big-O Analysis.*  
Accessed through: https://www.geeksforgeeks.org/dsa/analysis-algorithms-big-o-analysis/

[6] Rust Project Developers. *The Rust Programming Language.*  
Accessed through: https://rust-lang.org/

[7] GeeksforGeeks. *C Programming Language.*  
Accessed through: https://www.geeksforgeeks.org/c/c-programming-language/

[8] Lita Documentation. *Fibonacci: Rust vs. C Benchmarks.*  
Accessed through: https://lita.gitbook.io/lita-documentation/architecture/benchmarks/fibonacci-rust-vs.-c


