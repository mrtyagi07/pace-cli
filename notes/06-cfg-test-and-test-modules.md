# `#[cfg(test)]` and test modules

## The pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pace_formats_whole_minutes_per_km() {
        assert_eq!(format_pace(330), "5:30 /km");
    }
}
```

## `#[cfg(test)]` — conditional compilation

`#[cfg(test)]` is an attribute that means **"only compile this when running `cargo test`."**

- `cargo build` / `cargo run` → this whole module is invisible.
- `cargo test` → this module is compiled and the test runner finds it.

It doesn't bloat the release binary, doesn't slow normal builds, doesn't ship to users. Like `if (NODE_ENV === 'test')` — except the compiler enforces it at build time instead of leaving dead code in production.

## `mod tests`

Declares a **child module** named `tests`. It's just a namespace inside the file. The name `tests` is convention, not required.

## `use super::*;`

`super` = the parent module. `*` = everything. Together: **"bring all items from the parent into scope."**

This is why tests can call `format_pace(...)` directly instead of `super::format_pace(...)` or `crate::format_pace(...)`.

## `#[test]`

Marks a function as a test. When you run `cargo test`, the runner finds every `#[test]`-annotated function and runs it.

A test passes if it returns normally. It fails if it panics (e.g. from `assert!`, `assert_eq!`, `unwrap()` on an `Err`).

## `assert_eq!`

```rust
assert_eq!(format_pace(330), "5:30 /km");
```

Macro that checks if the two arguments are equal. On failure, it panics with a message showing both values (using `Debug` formatting), which fails the test and prints a clear diff.

Related: `assert!(condition)`, `assert_ne!(a, b)` (not equal).
