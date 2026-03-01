# Fuzzing Guide for physdes-rs

This document explains how to set up and run fuzzing tests for physdes-rs.

## What is Fuzzing?

Fuzzing is a testing technique that automatically generates random inputs to find bugs, crashes, and edge cases in code that might be missed by traditional testing.

## Installation

### Install cargo-fuzz

```bash
cargo install cargo-fuzz
```

### Install Python and pre-commit (optional but recommended)

```bash
# On Windows
# Download Python from https://www.python.org/downloads/
pip install pre-commit
```

## Running Fuzzers

### Run a specific fuzzer

```bash
# Fuzz polygon operations
cargo fuzz run polygon_ops

# Fuzz interval operations
cargo fuzz run interval_ops

# Fuzz point operations
cargo fuzz run point_ops
```

### Run fuzzers with a corpus

You can use existing test inputs as a corpus to guide the fuzzer:

```bash
cargo fuzz run polygon_ops fuzz/corpus/polygon_ops
```

### Run fuzzer for a specific duration

```bash
# Run for 60 seconds
cargo fuzz run polygon_ops -- -max_total_time=60

# Run for 5 minutes
cargo fuzz run interval_ops -- -max_total_time=300
```

### Minimize the corpus

After fuzzing, you can minimize the corpus to keep only interesting inputs:

```bash
cargo fuzz tmin interval_ops
```

## Available Fuzzers

### 1. polygon_ops

Tests polygon-related operations:
- Polygon creation
- Area calculation
- Convexity checking
- Bounding box computation
- Rectilinear checking

### 2. interval_ops

Tests interval-related operations:
- Interval creation
- Length calculation
- Overlap detection
- Intersection computation
- Convex hull calculation

### 3. point_ops

Tests point-related operations:
- Point creation
- Point arithmetic
- Comparison operations
- Distance calculation

## Understanding Fuzzer Output

### Normal Operation

The fuzzer will display:
- Number of executions per second
- Current corpus size
- Coverage statistics
- Unique crashes/findings

### Finding a Crash

If the fuzzer finds a crash, it will:
1. Save the crashing input to `fuzz/artifacts/`
2. Display the crash location
3. Show the input that caused the crash

### Example Crash Output

```
==9432== ERROR: libFuzzer: deadly signal
    #0 0x5555555a7e0f in std::sys::backtrace::tracing::imp::write::h423e5a2f5a9d3c5d
    #1 0x5555555a7e0f in std::panicking::default_hook::hb0f9e8c6d5e8a5c7
    ...
SUMMARY: libFuzzer: deadly signal
artifact_prefix='fuzz/artifacts/'; Test unit written to fuzz/artifacts/crash-<id>
```

## Fixing Issues Found by Fuzzing

1. **Reproduce the crash**:
   ```bash
   cargo fuzz run <fuzzer_name> fuzz/artifacts/crash-<id>
   ```

2. **Debug the issue**:
   - Use `gdb` or `lldb` to debug
   - Add logging to understand the failing case
   - Write a unit test for the specific input

3. **Fix the bug**:
   - Add proper validation
   - Handle edge cases
   - Add error handling where appropriate

4. **Verify the fix**:
   ```bash
   cargo test
   cargo fuzz run <fuzzer_name>
   ```

5. **Add to corpus**:
   Move the minimized input to the corpus to prevent regression:
   ```bash
   cp fuzz/artifacts/minimized-<id> fuzz/corpus/<fuzzer_name>/
   ```

## Continuous Fuzzing

### With pre-commit hooks

Add to `.git/hooks/pre-commit` (or use the provided `.pre-commit-config.yaml`):

```bash
#!/bin/bash
# Run fuzzers for a short time before committing
cargo fuzz run polygon_ops -- -max_total_time=10
cargo fuzz run interval_ops -- -max_total_time=10
```

### With CI/CD

Add to your CI configuration (`.github/workflows/fuzz.yml`):

```yaml
name: Fuzz
on: [push, pull_request]
jobs:
  fuzz:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo install cargo-fuzz
      - run: cargo fuzz run polygon_ops -- -max_total_time=60
      - run: cargo fuzz run interval_ops -- -max_total_time=60
```

## Best Practices

1. **Start with short runs**: Begin with 10-30 second runs to catch obvious bugs
2. **Use corpora**: Maintain a good corpus of interesting test cases
3. **Regular fuzzing**: Run fuzzers regularly, especially after code changes
4. **Address findings**: Fix issues found by fuzzers promptly
5. **Add unit tests**: Convert interesting fuzz findings into unit tests

## Troubleshooting

### Fuzzer won't run

- Ensure `cargo-fuzz` is installed
- Check that Rust is up to date
- Verify the fuzzer target exists

### Fuzzer is slow

- Reduce the number of test cases in corpus
- Use `-jobs=N` to run multiple fuzzer instances
- Limit fuzzing time with `-max_total_time`

### Out of memory

- Use `-rss_limit_mb=256` to limit memory usage
- Reduce corpus size
- Run fuzzer for shorter periods

## Additional Resources

- [libFuzzer documentation](https://llvm.org/docs/LibFuzzer.html)
- [cargo-fuzz book](https://rust-fuzz.github.io/book/cargo-fuzz.html)
- [Google OSS-Fuzz](https://github.com/google/oss-fuzz)