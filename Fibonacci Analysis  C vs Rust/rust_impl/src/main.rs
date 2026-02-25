//! Rust - Fibonacci Sequence
//!
//! In this program, I implemented three different methods to compute
//! the nth Fibonacci number:
//! 1) Iterative
//! 2) Recursive
//! 3) Dynamic Programming (tabulation)
//!
//! For each method, I measure:
//! - Execution time
//! - Number of addition operations performed
//!
//! The program runs one value of n at a time.
//! I can optionally repeat the computation multiple times
//! to obtain more stable empirical timing results.
//!
//! Note: Fibonacci values are stored as u64.
//! u64 overflows after n = 93.

use std::env;
use std::time::Instant;

/// This struct stores the results of running one Fibonacci algorithm.
/// I use it to return multiple pieces of information together.
struct FibRun {
    value: u64,
    ops: u64,
    seconds: f64,
}

/// Iterative Fibonacci implementation.
///
/// I build the sequence from the bottom up.
/// Only the last two values are stored at any time.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
fn fib_iter(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    let mut a = 0u64; // Represents F(i-2)
    let mut b = 1u64; // Represents F(i-1)

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

/// Recursive Fibonacci implementation.
///
/// This directly follows the mathematical definition:
/// F(n) = F(n-1) + F(n-2)
///
/// This version is inefficient because it recomputes
/// the same subproblems many times.
///
/// Time Complexity: O(2^n)
/// Space Complexity: O(n)
fn fib_recur(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    fib_recur(n - 1) + fib_recur(n - 2)
}

/// Dynamic Programming Fibonacci implementation.
///
/// This version stores previously computed values in a vector.
/// Each value is computed exactly once.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
fn fib_dp(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    let mut table = vec![0u64; (n + 1) as usize];
    table[0] = 0;
    table[1] = 1;

    for i in 2..=n {
        let idx = i as usize;
        table[idx] = table[idx - 1] + table[idx - 2];
    }

    table[n as usize]
}

/// Runs and times the iterative version while counting additions.
fn run_iterative(n: u64) -> FibRun {
    let start = Instant::now();
    let mut ops = 0u64;

    let value = if n < 2 {
        n
    } else {
        let mut a = 0u64;
        let mut b = 1u64;

        for _ in 2..=n {
            let temp = a + b;
            ops += 1; // Count each addition
            a = b;
            b = temp;
        }
        b
    };

    let seconds = start.elapsed().as_secs_f64();
    FibRun { value, ops, seconds }
}

/// Helper function for recursive version that counts additions.
fn fib_rec_count(n: u64, ops: &mut u64) -> u64 {
    if n < 2 {
        return n;
    }

    let a = fib_rec_count(n - 1, ops);
    let b = fib_rec_count(n - 2, ops);

    *ops += 1; // Count the addition operation
    a + b
}

/// Runs and times the recursive version.
fn run_recursive(n: u64) -> FibRun {
    let start = Instant::now();
    let mut ops = 0u64;

    let value = fib_rec_count(n, &mut ops);

    let seconds = start.elapsed().as_secs_f64();
    FibRun { value, ops, seconds }
}

/// Runs and times the dynamic programming version.
fn run_dp(n: u64) -> FibRun {
    let start = Instant::now();
    let mut ops = 0u64;

    let value = if n < 2 {
        n
    } else {
        let mut table = vec![0u64; (n + 1) as usize];
        table[0] = 0;
        table[1] = 1;

        for i in 2..=n {
            let idx = i as usize;
            table[idx] = table[idx - 1] + table[idx - 2];
            ops += 1; // Count each addition
        }

        table[n as usize]
    };

    let seconds = start.elapsed().as_secs_f64();
    FibRun { value, ops, seconds }
}

/// Repeats an algorithm multiple times.
/// This helps produce more stable timing results
/// when runtimes are extremely small.
fn repeat_run<F>(mut func: F, n: u64, reps: u64) -> FibRun
where
    F: FnMut(u64) -> FibRun,
{
    let mut total_time = 0.0;
    let mut last_ops = 0;
    let mut last_value = 0;

    for _ in 0..reps {
        let result = func(n);
        total_time += result.seconds;
        last_ops = result.ops;
        last_value = result.value;
    }

    FibRun {
        value: last_value,
        ops: last_ops,
        seconds: total_time / reps as f64,
    }
}

/// Main function.
///
/// This program expects the following command line arguments:
///
///     <n> <algorithm> [--reps X]
///
/// n:
///     The Fibonacci value to compute.
///
/// algorithm:
///     0 = iterative
///     1 = recursive
///     2 = dynamic programming
///     3 = all three algorithms
///     4 = iterative + dynamic programming (recursion skipped)
///
/// --reps X (optional):
///     Repeats the computation X times and averages the runtime.
///     This helps produce more stable empirical timing data
///     when runtimes are extremely small.
///
/// Output format (CSV style):
///
///     iter_time,iter_ops,dp_time,dp_ops,rec_time,rec_ops
///
/// If an algorithm is not executed, its time and ops are printed as 0.
fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // We require at least <program> <n> <algorithm>
    if args.len() < 3 {
        eprintln!("Usage: <program> <n> <algorithm> [--reps X]");
        std::process::exit(1);
    }

    // Parse n
    let n: u64 = args[1]
        .parse()
        .expect("Invalid value for n. Must be a positive integer.");

    // Parse algorithm selection
    let algorithm: u32 = args[2]
        .parse()
        .expect("Invalid algorithm selection.");

    // Default repetitions = 1
    let mut reps: u64 = 1;

    // If user provided --reps X
    if args.len() >= 5 && args[3] == "--reps" {
        reps = args[4].parse().expect("Invalid value for reps.");
    }

    match algorithm {
        // 0 = Iterative only
        0 => {
            let result = repeat_run(run_iterative, n, reps);
            println!(
                "{:.9},{},{:.9},{},{:.9},{}",
                result.seconds, result.ops,
                0.0, 0,
                0.0, 0
            );
        }

        // 1 = Recursive only
        1 => {
            let result = repeat_run(run_recursive, n, reps);
            println!(
                "{:.9},{},{:.9},{},{:.9},{}",
                0.0, 0,
                0.0, 0,
                result.seconds, result.ops
            );
        }

        // 2 = Dynamic Programming only
        2 => {
            let result = repeat_run(run_dp, n, reps);
            println!(
                "{:.9},{},{:.9},{},{:.9},{}",
                0.0, 0,
                result.seconds, result.ops,
                0.0, 0
            );
        }

        // 3 = All three algorithms (recursion may be slow for large n)
        3 => {
            let iter = repeat_run(run_iterative, n, reps);
            let dp = repeat_run(run_dp, n, reps);
            let rec = repeat_run(run_recursive, n, reps);

            println!(
                "{:.9},{},{:.9},{},{:.9},{}",
                iter.seconds, iter.ops,
                dp.seconds, dp.ops,
                rec.seconds, rec.ops
            );
        }

        // 4 = Iterative + DP only (no recursion)
        4 => {
            let iter = repeat_run(run_iterative, n, reps);
            let dp = repeat_run(run_dp, n, reps);

            println!(
                "{:.9},{},{:.9},{},{:.9},{}",
                iter.seconds, iter.ops,
                dp.seconds, dp.ops,
                0.0, 0
            );
        }

        _ => {
            eprintln!("Invalid algorithm selection. Use 0,1,2,3,4.");
            std::process::exit(1);
        }
    }
}

