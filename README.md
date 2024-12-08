# Advent of Code 2024

This repository contains my solutions for Advent of Code 2024.

The input for day `N` is stored in `data/dayN.txt`.
Per AoC guidelines, I don't commit these input files to GitHub.

To run the solution for $DAY (1,2,3...) and $PROBLEM (1 or 2):
```
cargo run -r $DAY $PROBLEM
```

To run all the unit tests:
```
cargo t -r
```

For completed days, the unit tests include verifying the overall solution is correct
    and thus the input files need to be present.
There are also unit tests for the examples in the problem statements,
    and sometimes other things.
The flag `-r` is needed because some of the solutions are brute-force and are
    too slow in the default debug mode.
