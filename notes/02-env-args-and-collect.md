# `env::args().collect()`

```rust
let args: Vec<String> = env::args().collect();
```

## What's happening

- `env::args()` returns an **iterator** over command-line arguments.
- `.collect()` consumes the iterator and bundles the items into a collection — here, a `Vec<String>`.

The type annotation `Vec<String>` tells `collect()` which container to build. Without it, Rust can't infer the type because `collect()` is generic.

## Indexing

For `pace-cli pace 330`:

| Index | Value |
|-------|-------|
| `args[0]` | `"pace-cli"` (program name) |
| `args[1]` | `"pace"` (first user arg) |
| `args[2]` | `"330"` (second user arg) |

**Gotcha**: `args[0]` is always the program name, not user input. Real user args start at index 1.

## Why an iterator first?

Iterators are lazy — they don't allocate until you consume them. `.collect()` is the consumption step that produces the final `Vec`.
