# The `?` operator and `.map_err()`

## `?` — early return on error

```rust
let value: u32 = args[2].parse()?;
```

`?` unwraps a `Result`:
- If `Ok(value)` → give me `value`, keep going.
- If `Err(e)` → **return early** from the whole function with that error.

It's syntactic sugar for:
```rust
let value: u32 = match args[2].parse() {
    Ok(v) => v,
    Err(e) => return Err(e.into()),
};
```

**Constraint**: `?` only works in functions that themselves return `Result` (or `Option`). That's why `main`'s signature is `Result<(), Box<dyn Error>>` instead of just `()`.

## `.map_err()` — transform the error case

`parse()` returns a generic `ParseIntError`, which isn't friendly. We want a custom message.

```rust
let value: u32 = args[2].parse()
    .map_err(|_| format!("'{}' is not a valid non-negative integer", args[2]))?;
```

- If `parse()` returns `Ok(value)` → `map_err` does nothing, we get `Ok(value)`.
- If `parse()` returns `Err(e)` → `map_err` takes `e`, ignores it (`|_|`), and produces a **new error** with our custom message.
- `?` then propagates that new error up to the caller of `main`.

## Mental model

```
parse()            →  Result<u32, ParseIntError>
.map_err(|_| ...)  →  Result<u32, String>          (error type transformed)
?                  →  u32 (or early return with the String error)
```

`map_err` only touches the error branch — `Ok` values flow through untouched.
