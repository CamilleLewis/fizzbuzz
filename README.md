# fizzbuzz

A Rust implementation of fizzbuzz.

## Run

Run with default values by calling `cargo run`. Default values are 3 for fizz, 5 for buzz, and 15 to iterate to.

Optional values to configure the fizzbuzz function can be specified with `-f` to specify the fizz divisor, `-b` to specify the buzz divisor, and `-n` to specify the number to iterate to.

To run with custom values, call `cargo run -- {args}`.

## Test

Tests are run with `cargo test`. See, I TDD too.