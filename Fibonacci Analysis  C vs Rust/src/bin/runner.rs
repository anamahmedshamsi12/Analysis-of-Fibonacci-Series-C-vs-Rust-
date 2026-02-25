use std::process::{Command, Stdio};
use std::time::Duration;
use std::fs::File;
use std::io::{Write, BufWriter};
use wait_timeout::ChildExt;

const TIMEOUT: u64 = 60;
const EXEC: &str = "./target/release/fib_project"; // actual built binary

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

    let mut run_type = 3u32;

    for n in (1..=max_n).step_by(step as usize) {
        match run_single(n, run_type) {
            Ok((timings, ops)) => {
                writeln!(timings_writer, "{},{}", n, timings.join(",")).unwrap();
                writeln!(ops_writer, "{},{}", n, ops.join(",")).unwrap();
            }
            Err(_) => {
                // mimic python: if recursion times out, switch type and retry
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