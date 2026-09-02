# LeetCode in Rust

Practicing LeetCode to learn Rust from the ground up. One library crate,
one module per problem, tests inline. `cargo test` is the run button.

## Structure

```
src/
├── lib.rs              # module list
├── common.rs           # ListNode, TreeNode, shared helpers
└── pNNNN_slug.rs       # one file per problem
```

Each problem file holds the solution plus a `#[cfg(test)] mod tests` block.

## Cargo commands

| Command | What it does |
| --- | --- |
| `cargo check` | Type-checks without producing a binary. Fast inner loop while solving. |
| `cargo test` | Compiles and runs every problem's tests. |
| `cargo test p0001` | Runs only tests whose path matches `p0001`. |
| `cargo test -- --nocapture` | Runs tests but lets `println!` output through (hidden by default on pass). |
| `cargo build` | Compiles the whole crate (debug). Rarely needed directly for practice. |
| `cargo run --bin p0001` | Runs a `src/bin/p0001.rs` with a `main` — for problems you want to execute, not assert. |
| `cargo clippy` | Lint pass. Catches non-idiomatic Rust — worth running as you learn. |
| `cargo fmt` | Auto-formats to the standard style. |
| `cargo add <crate>` | Adds a dependency and edits `Cargo.toml`. |

## Adding a problem

1. Create `src/pNNNN_slug.rs` with the solution and tests.
2. Add `pub mod pNNNN_slug;` to `src/lib.rs`.
3. `cargo test pNNNN` to check it.