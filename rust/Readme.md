# *Rust* *Slurm* tests

Minimal set of *Slurm* tests added to a `cargo` project.

A `runtests` test runner is provided which accepts on the command line a path to the location of the tests.

As for any `cargo` project all executable are found inside the `target/debug` or `target/release` directory.

To run with `cargo` from the top-level project directory:

```shell
cargo run --bin rutests -- ./target/debug

```

To achieve full portability of the generated executables select
