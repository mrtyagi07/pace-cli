# Useful `cargo test` flags

## Filter which tests run

```bash
cargo test pace                  # run tests whose names contain "pace"
cargo test pace_formats          # more specific
cargo test --test integration    # run only the integration test file named "integration"
```

Rust filters by substring match against test names.

## `-- --test-threads=1`

```bash
cargo test -- --test-threads=1
```

By default, Rust runs tests in parallel for speed. If your tests have shared side effects (writing to the same file, hitting the same DB), they'll race. `--test-threads=1` forces sequential execution.

The bare `--` separates `cargo`'s flags from flags passed to the test binary.

## `-- --nocapture`

```bash
cargo test -- --nocapture
```

By default, Rust **captures all stdout from tests** and only shows it if the test fails. With `--nocapture`, you see `println!` output even from passing tests — useful for debugging.

## `-- --ignored`

```bash
cargo test -- --ignored
```

Runs tests marked `#[ignore]` (typically slow or flaky ones excluded from default runs).

## `-- --exact`

```bash
cargo test pace_formats_whole_minutes_per_km -- --exact
```

Match the test name exactly instead of as a substring.

## Combine

```bash
cargo test pace -- --nocapture --test-threads=1
```

Filter to "pace" tests, show output, run sequentially.
