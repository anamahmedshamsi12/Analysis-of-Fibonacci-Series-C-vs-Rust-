/*
 * Fibonacci - C Implementation
 *
 * In this program, I implemented three approaches to compute
 * the nth Fibonacci number:
 *
 *   1) Iterative
 *   2) Recursive
 *   3) Dynamic Programming (Tabulation)
 *
 * For empirical analysis, I measure:
 *   - Execution time (seconds)
 *   - Number of addition operations performed
 *
 * This program is designed to be run from the command line:
 *
 *   ./fib_c <n> <algorithm> [--reps X]
 *
 * algorithm:
 *   0 = iterative
 *   1 = recursive
 *   2 = dynamic programming
 *   3 = all three algorithms
 *   4 = iterative + dynamic programming (recursion skipped)
 *
 * Output format (CSV style):
 *   iter_time,iter_ops,dp_time,dp_ops,rec_time,rec_ops
 *
 * NOTE:
 * Fibonacci values are stored as unsigned long long.
 * This overflows after n = 93.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

/*
 * I use this struct to return multiple pieces of information
 * from each algorithm:
 *   - computed Fibonacci value
 *   - number of addition operations
 *   - runtime (seconds)
 */
typedef struct {
    unsigned long long value;
    unsigned long long ops;
    double seconds;
} FibRun;

/*
 * Helper function to get current time in seconds.
 *
 * I use clock() because it is portable and simple for class use.
 * clock() measures CPU time used by the program (not wall time).
 */
static double current_time_seconds(void) {
    return (double)clock() / (double)CLOCKS_PER_SEC;
}

/*
 * Helper recursive function that counts addition operations.
 *
 * Each time I compute a + b, I increment ops.
 * This function is intentionally expensive for large n
 * due to overlapping subproblems.
 */
static unsigned long long fib_rec_count(unsigned long long n,
                                        unsigned long long *ops) {
    if (n < 2ULL) {
        return n;
    }

    unsigned long long a = fib_rec_count(n - 1ULL, ops);
    unsigned long long b = fib_rec_count(n - 2ULL, ops);

    (*ops)++;  /* count one addition */
    return a + b;
}

/*
 * Iterative Fibonacci implementation.
 *
 * I build the sequence from the bottom up,
 * only storing the previous two values.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */
static FibRun run_iterative(unsigned long long n) {
    FibRun result;
    result.value = 0ULL;
    result.ops = 0ULL;
    result.seconds = 0.0;

    double start = current_time_seconds();

    if (n < 2ULL) {
        result.value = n;
    } else {
        unsigned long long a = 0ULL;
        unsigned long long b = 1ULL;

        for (unsigned long long i = 2ULL; i <= n; i++) {
            unsigned long long temp = a + b;
            result.ops++;     /* count one addition */
            a = b;
            b = temp;
        }

        result.value = b;
    }

    double end = current_time_seconds();
    result.seconds = end - start;

    return result;
}

/*
 * Recursive Fibonacci implementation.
 *
 * This follows the mathematical definition:
 *   F(n) = F(n-1) + F(n-2)
 *
 * Time Complexity: O(2^n)
 * Space Complexity: O(n)
 */
static FibRun run_recursive(unsigned long long n) {
    FibRun result;
    result.value = 0ULL;
    result.ops = 0ULL;
    result.seconds = 0.0;

    double start = current_time_seconds();
    result.value = fib_rec_count(n, &result.ops);
    double end = current_time_seconds();

    result.seconds = end - start;
    return result;
}

/*
 * Dynamic Programming Fibonacci implementation.
 *
 * I allocate an array to store all values from F(0) to F(n).
 * Each subproblem is computed exactly once.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */
static FibRun run_dp(unsigned long long n) {
    FibRun result;
    result.value = 0ULL;
    result.ops = 0ULL;
    result.seconds = 0.0;

    double start = current_time_seconds();

    if (n < 2ULL) {
        result.value = n;
    } else {
        /* Allocate a table of size n+1 */
        unsigned long long *table =
            (unsigned long long *)malloc((n + 1ULL) * sizeof(unsigned long long));

        if (table == NULL) {
            fprintf(stderr, "Memory allocation failed\n");
            exit(1);
        }

        table[0] = 0ULL;
        table[1] = 1ULL;

        for (unsigned long long i = 2ULL; i <= n; i++) {
            table[i] = table[i - 1ULL] + table[i - 2ULL];
            result.ops++;  /* count one addition */
        }

        result.value = table[n];
        free(table);
    }

    double end = current_time_seconds();
    result.seconds = end - start;

    return result;
}

/*
 * Main function
 *
 * Parses command line arguments and runs the requested algorithm(s).
 * If --reps is provided, I repeat the run multiple times and average
 * the runtime (operations and value come from the last run).
 *
 * Output is always printed as:
 *   iter_time,iter_ops,dp_time,dp_ops,rec_time,rec_ops
 *
 * If recursion is not executed (algorithm 4), recursion is printed as 0.
 */
int main(int argc, char *argv[]) {

    if (argc < 3) {
        fprintf(stderr, "Usage: ./fib_c <n> <algorithm> [--reps X]\n");
        return 1;
    }

    unsigned long long n = strtoull(argv[1], NULL, 10);
    int algorithm = atoi(argv[2]);

    /* default repetitions */
    unsigned long long reps = 1ULL;

    if (argc == 5 && strcmp(argv[3], "--reps") == 0) {
        reps = strtoull(argv[4], NULL, 10);
        if (reps == 0ULL) reps = 1ULL;
    }

    /* Store last-run results (so we can print ops/value). */
    FibRun it = (FibRun){0ULL, 0ULL, 0.0};
    FibRun dp = (FibRun){0ULL, 0ULL, 0.0};
    FibRun rec = (FibRun){0ULL, 0ULL, 0.0};

    /* Accumulate times so we can average. */
    double it_sum = 0.0;
    double dp_sum = 0.0;
    double rec_sum = 0.0;

    /*
     * Run the selected algorithm(s) reps times.
     * This helps stabilize timing measurements for small runtimes.
     */
    for (unsigned long long r = 0ULL; r < reps; r++) {

        /* iterative executes for algorithm 0, 3, or 4 */
        if (algorithm == 0 || algorithm == 3 || algorithm == 4) {
            it = run_iterative(n);
            it_sum += it.seconds;
        }

        /* dp executes for algorithm 2, 3, or 4 */
        if (algorithm == 2 || algorithm == 3 || algorithm == 4) {
            dp = run_dp(n);
            dp_sum += dp.seconds;
        }

        /* recursion executes only for algorithm 1 or 3 */
        if (algorithm == 1 || algorithm == 3) {
            rec = run_recursive(n);
            rec_sum += rec.seconds;
        }
    }

    /* Compute average seconds for whichever algorithms ran. */
    if (algorithm == 0 || algorithm == 3 || algorithm == 4) {
        it.seconds = it_sum / (double)reps;
    } else {
        it.seconds = 0.0;
        it.ops = 0ULL;
    }

    if (algorithm == 2 || algorithm == 3 || algorithm == 4) {
        dp.seconds = dp_sum / (double)reps;
    } else {
        dp.seconds = 0.0;
        dp.ops = 0ULL;
    }

    if (algorithm == 1 || algorithm == 3) {
        rec.seconds = rec_sum / (double)reps;
    } else {
        rec.seconds = 0.0;
        rec.ops = 0ULL;
    }

    /*
     * Print a single CSV line:
     * iter_time,iter_ops,dp_time,dp_ops,rec_time,rec_ops
     *
     * I print 9 decimal places to match the Rust output style.
     */
    printf("%.9f,%llu,%.9f,%llu,%.9f,%llu\n",
           it.seconds, it.ops,
           dp.seconds, dp.ops,
           rec.seconds, rec.ops);

    return 0;
}