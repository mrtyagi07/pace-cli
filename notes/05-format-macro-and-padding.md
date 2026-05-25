# `format!` and the `{:02}` zero-padding spec

## `format!` basics

`format!` builds a `String` by interpolating values into a template:

```rust
let s = format!("{}:{}", minutes, seconds);  // e.g. "5:5"
```

Placeholders `{}` use the value's default `Display` implementation.

## Format spec inside `{}`

You can customize how a value is rendered by adding a **format spec** after a colon:

```
{:[fill][align][sign][#][0][width][.precision][type]}
```

## `{:02}` decoded

```rust
format!("{}:{:02} /km", minutes, seconds);
```

- `0` → pad with zeros (instead of spaces)
- `2` → minimum width of 2 characters

So:
- `seconds = 5`  → `"05"`
- `seconds = 12` → `"12"`
- `seconds = 0`  → `"00"`

This ensures pace always renders as `M:SS` — never `5:5` or `5:0`.

## Other useful specs

| Spec | Effect | Example |
|------|--------|---------|
| `{:5}` | width 5, space-padded right-aligned | `"   42"` |
| `{:<5}` | width 5, left-aligned | `"42   "` |
| `{:>5}` | width 5, right-aligned (default) | `"   42"` |
| `{:.2}` | 2 decimal places | `"3.14"` |
| `{:x}` | hex | `"ff"` |
| `{:b}` | binary | `"11111111"` |
| `{:?}` | Debug formatting | `Some(5)` |
| `{:#?}` | pretty Debug | multi-line |

## Related macros

- `format!` → returns `String`
- `println!` → prints to stdout
- `eprintln!` → prints to stderr
- `write!` → writes into a buffer

All use the same format spec syntax.
