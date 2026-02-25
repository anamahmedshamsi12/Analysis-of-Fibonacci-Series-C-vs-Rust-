import csv
import subprocess

EXEC = "./fib"
N_MAX = 5000
ALG = "4"  # iterative + dp only, recursion skipped

TIMINGS_OUT = "timings_c.csv"
OPS_OUT = "ops_c.csv"
HEADER = ["N", "Iterative", "Dynamic Programming", "Recursive"]

def run_one(n: int):
    # fib prints: it_time,it_ops,dp_time,dp_ops,rec_time,rec_ops
    out = subprocess.check_output([EXEC, str(n), ALG], text=True).strip()
    it_t, it_o, dp_t, dp_o, rec_t, rec_o = out.split(",")

    timings = [float(it_t), float(dp_t), float(rec_t)]
    ops = [int(it_o), int(dp_o), int(rec_o)]
    return timings, ops

def main():
    with open(TIMINGS_OUT, "w", newline="") as ft, open(OPS_OUT, "w", newline="") as fo:
        tw = csv.writer(ft)
        ow = csv.writer(fo)

        tw.writerow(HEADER)
        ow.writerow(HEADER)

        for n in range(1, N_MAX + 1):
            timings, ops = run_one(n)

            # Format timings to fixed decimal
            formatted_timings = [
                f"{timings[0]:.6f}",
                f"{timings[1]:.6f}",
                "-" if timings[2] == 0.0 else f"{timings[2]:.6f}"
            ]

            tw.writerow([n] + formatted_timings)
            ow.writerow([n] + ops)

            if n % 250 == 0:
                print(f"wrote n={n}")

if __name__ == "__main__":
    main()