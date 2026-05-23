# pace-cli

A tiny Rust CLI for formatting running pace and durations.

## Usage

```bash
pace-cli <pace|dur> <seconds>
```

- `pace <seconds>` — format seconds-per-km as `M:SS /km`
- `dur <seconds>` — format a duration as `Hh Mm Ss` (drops zero leading units)

## Examples

```bash
$ pace-cli pace 330
5:30 /km

$ pace-cli pace 305
5:05 /km

$ pace-cli dur 5025
1h 23m 45s

$ pace-cli dur 125
2m 5s

$ pace-cli dur 30
30s
```

## Build & run

```bash
cargo build --release
cargo run -- pace 330
```

## Test

```bash
cargo test
```
