/*
Rust Runner
Overview
This program is a benchmarking driver. It repeatedly runs a compiled Fibonacci
executable ("./target/release/fib_project") with different input sizes (N),
collects the timing and operation-count results it prints, and writes those
results into two CSV files:

1) timings_fib.csv  -> runtime measurements
2) ops_fib.csv      -> operation-count measurements

The goal is to automate performance testing across many input sizes and safely
handle cases where certain implementations become too slow.

Key Design Decisions
I use std::process::Command to spawn the Fibonacci executable as a child process.
I enforce a timeout using the wait_timeout crate to prevent very slow runs from
hanging the entire benchmark.
I write results using BufWriter for efficient file output since this program
writes thousands of lines.
I assume the executable prints comma-separated output in pairs:
    time1, ops1, time2, ops2, ...
and I parse that accordingly.

Constants

TIMEOUT:
Maximum number of seconds to wait for a single execution before killing it.

EXEC:
Path to the compiled Fibonacci binary that this program runs.

Function: run_single

Signature:
fn run_single(n: u32, typ: u32)
    -> Result<(Vec<String>, Vec<String>), String>

Purpose:
Runs the Fibonacci executable once with input n and run mode typ.
Returns:
    - A vector of timing values
    - A vector of operation-count values
or an error string if something fails.

Steps:
1. Spawn the executable with arguments (n, typ).
2. Pipe stdout and stderr so they can be captured.
3. Wait up to TIMEOUT seconds.
4. If it finishes:
   - Check if exit status is successful.
   - Read stdout.
   - Split the output by commas.
   - Validate that the number of fields is even.
   - Separate the fields into two vectors:
         timings[] and ops[]
5. If it times out:
   - Kill the process.
   - Return an error.

I validate that the number of comma-separated fields is even because the output
must come in (time, ops) pairs. If it does not, that indicates malformed output.

Function: main

Configuration:
max_n = 5000
step  = 1

This means the program benchmarks every N from 1 to 5000.

File Output:
Two CSV files are created:
    timings_fib.csv
    ops_fib.csv

Each file starts with a header:
    N,Iterative,Dynamic Programming,Recursive

Loop Logic:
The program starts with run_type = 3 (which includes recursion).
For each N:
    It calls run_single(n, run_type).

If it succeeds:
    It writes N and the results to both CSV files.

If it fails (typically due to timeout):
    It switches run_type to 4.
    It retries the same N without recursion.
    It continues using run_type = 4 afterward.

This ensures:
- The benchmark continues even if recursion becomes too slow.
- Data remains aligned by N.
- The program does not hang.

Performance Considerations:
BufWriter reduces disk I/O overhead.
The timeout prevents exponential recursive behavior from freezing the benchmark.
Using a child process isolates the benchmark logic from the executable.

Final Output:
When finished, the program prints:
    "Done. Generated timings_fib.csv and ops_fib.csv"

These CSV files can then be used for graphing or analysis.
*/

use std::process::{Command, Stdio};
use std::time::Duration;
use std::fs::File;
use std::io::{Write, BufWriter};
use wait_timeout::ChildExt;

const TIMEOUT: u64 = 60;
const EXEC: &str = "./target/release/fib_project"; // <-- this matches your binary name

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

            if parts.len() % 2 != 0 {
                return Err(format!("Malformed output: {}", line.trim()));
            }

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
            let _ = child.wait();
            Err("Timeout reached".to_string())
        }
    }
}

fn main() {
    let max_n = 5000u32;
    let step = 1u32;

    let timings_file = File::create("timings_fib.csv").unwrap();
    let ops_file = File::create("ops_fib.csv").unwrap();

    let mut timings_writer = BufWriter::new(timings_file);
    let mut ops_writer = BufWriter::new(ops_file);

    writeln!(timings_writer, "N,Iterative,Dynamic Programming,Recursive").unwrap();
    writeln!(ops_writer, "N,Iterative,Dynamic Programming,Recursive").unwrap();

    // Start with recursion enabled (like the Python runner). If it times out, switch modes.
    let mut run_type = 4u32;

    for n in (1..=max_n).step_by(step as usize) {
        match run_single(n, run_type) {
            Ok((timings, ops)) => {
                writeln!(timings_writer, "{},{}", n, timings.join(",")).unwrap();
                writeln!(ops_writer, "{},{}", n, ops.join(",")).unwrap();
            }
            Err(_) => {
                // If recursion times out, switch mode and retry for the same n.
                run_type = 4;
                if let Ok((timings, ops)) = run_single(n, run_type) {
                    writeln!(timings_writer, "{},{}", n, timings.join(",")).unwrap();
                    writeln!(ops_writer, "{},{}", n, ops.join(",")).unwrap();
                }
            }
        }
    }

    println!("Done. Generated timings_fib.csv and ops_fib.csv");
}