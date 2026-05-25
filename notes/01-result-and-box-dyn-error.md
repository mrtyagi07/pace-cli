# `Result<(), Box<dyn std::error::Error>>`

The return type of `main` in this project. Translated:
**"This function either succeeds with nothing, or fails with some error — and I don't care exactly which kind."**

## Piece by piece

### `Result<OK, ERR>`
A box with two slots. Only one is filled:
- `Ok(...)` → success
- `Err(...)` → failure

### `()`
The "unit type" — basically "nothing". Like `void` in C/TS.
Means "success carries no value, just the fact that it worked."

### `Box<...>`
A pointer to data on the heap.

```
Stack:  [Box ptr] ────►  Heap: [actual error object]
```

### `dyn std::error::Error`
`dyn` = "dynamic dispatch". Means: "any type, as long as it implements the `Error` trait."

We don't know at compile time *which* error type it is (could be `io::Error`, `ParseIntError`, etc.) — only that it behaves like an error.

## Why heap (`Box`)?

Different error types have different sizes. `io::Error` might be 24 bytes, `ParseIntError` might be 4 bytes. The compiler needs a **fixed size** on the stack.

Solution: put the error on the heap (where size doesn't matter), and keep just a fixed-size pointer (`Box`) on the stack.

## Mailbox analogy

A `Result` is a **mailbox**:
- Empty → `Ok(())` → all good
- Has a letter → `Err(...)` → something went wrong

The letter could be any shape or size, so instead of stuffing it in the mailbox directly, you put a **claim ticket** (`Box`) in the mailbox. The actual letter sits in a warehouse (heap). The ticket says: "go fetch whatever error is stored there — I don't know what kind, but it's *some* error."

## Why use this in `main`?

It's the flexible choice — `main` can return *any* error type from `?` operators without you having to define a specific error enum.

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("foo.txt")?;  // io::Error
    let n: i32 = file.trim().parse()?;               // ParseIntError
    Ok(())
}
```

Both errors are different types, but both implement `Error` — so both fit in `Box<dyn Error>`.
