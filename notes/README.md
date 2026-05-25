# pace-cli learning notes

Personal notes on Rust concepts learned while building `pace-cli`. Indexed by topic so I can grep when the same question comes up in another project.

## Index

1. [Result and Box<dyn Error>](./01-result-and-box-dyn-error.md) — `main`'s flexible error return type
2. [env::args and collect](./02-env-args-and-collect.md) — reading CLI arguments
3. [The `?` operator and `.map_err()`](./03-question-mark-and-map-err.md) — error propagation
4. [String vs &str](./04-string-vs-str.md) — owned vs borrowed strings, `.as_str()`
5. [format! macro and padding](./05-format-macro-and-padding.md) — `{:02}` and other format specs
6. [#[cfg(test)] and test modules](./06-cfg-test-and-test-modules.md) — how Rust tests are wired
7. [Useful cargo test flags](./07-cargo-test-flags.md) — filtering, capture, threads
