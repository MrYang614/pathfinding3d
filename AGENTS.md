# Working on pathfinding3d

These guidelines apply throughout the repository. Use the general guidance in
the [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html)
for this Rust 2021 Cargo library. Sections marked for Chrome or Fuchsia are
outside the scope of this project.

## Rust changes

- Use stable Rust and let `cargo fmt` handle formatting. Keep refactors local
  to the code involved in the change.
- Import types by name and qualify calls to free functions from other modules.
  Group imports from the same module. Prefer explicit imports in production
  code. Inline tests may use `super::*`.
- Borrow read-only data as `&str` or `&[T]` when the caller retains ownership.
  Reuse the existing pathfinding scratch buffers in repeated queries.
- Use pattern matching or `?` for missing data and recoverable errors. When an
  invariant warrants `expect`, describe the invariant in the message.
- Document caller-visible behavior, including units, indexing and failure
  results. Give private helpers names that make their purpose clear.

## Compatibility

- Preserve public Rust signatures and paths, the JavaScript class and method
  names, and the typed arrays accepted and returned by the WASM bindings.
- Node IDs are local to a group. Pair them with the group used for the query.
- `find_path` returns a waypoint count. It skips the start and writes packed
  `x, y, z` triples into the caller's `Float32Array`, leaving the tail intact.
  A short buffer receives no writes and the caller gets the required waypoint
  count. A missing route returns zero and leaves the buffer intact. If the
  waypoint count exceeds `i32::MAX`, the method returns `i32::MIN` before writing.

## Verification

Run these checks from the repository root. The release check writes to
`target/` to keep the tracked npm package in `pkg/` intact.

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
wasm-pack build --release --target bundler --out-dir target/wasm-pkg
wasm-pack test --headless --chrome
```

Use fixed navigation meshes for browser regression tests. Exercise the public
bindings with a real JavaScript typed array and check both the return value and
the caller's buffer. The browser suite can also run with `--firefox`.
