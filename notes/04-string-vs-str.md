# `String` vs `&str` and `.as_str()`

## The two string types

| Type | What it is | Analogy |
|------|-----------|---------|
| `String` | Owns its text (heap-allocated, growable) | A book you own |
| `&str` | Borrows a view into existing text | A bookmark pointing into someone else's book |

## `.as_str()`

Converts a **`String`** into a **`&str`** — a borrowed view of the same characters. No copy, no allocation, just a pointer + length.

```
String (owned)  ──.as_str()──►  &str (borrowed view)
```

## Why we need it in the match

```rust
match args[1].as_str() {
    "pace" => ...,
    "dur"  => ...,
}
```

Match arms here use string literals (`"pace"`, `"dur"`), which are `&'static str`. You can't match a `String` directly against `&str` literals — `.as_str()` bridges the gap.

## Visual

```
args[1] (String):
  ┌─────────────────┐
  │ ptr ──► "pace"  │  ← owns the heap data
  │ len: 4          │
  │ cap: 4          │
  └─────────────────┘
         │
         │ .as_str()
         ▼
  ┌─────────────────┐
  │ ptr ──► "pace"  │  ← just borrows, points to same data
  │ len: 4          │
  └─────────────────┘
   (this is &str)
```

## Rule of thumb

- Function **takes** a string parameter? Prefer `&str` — it accepts both `&String` and string literals.
- Function **returns** a new string? Use `String` — caller owns it.
- Need to mutate/grow? `String`.
- Just reading? `&str`.
