# Minimal set of *Slurm* tests implemented in *Rust*

The *subprocess* module is used to interact with `salloc` and the other *Slurm* executables.

All tests are located inside the `src/bin` sub-directory.

A test runner, *runtests*, is provided taking the path to the location of the tests on the command line.

When building with `cargo` all executables are found in `target/release` and `target/debug` directories.

Running with *cargo*:
```shell
cargo run --bin runtests -- ./target/debug

```
