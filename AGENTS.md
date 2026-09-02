# Repository Guidelines

## Project Structure & Module Organization

This repository is a Rustlings practice workspace. Exercises are grouped by topic under `exercises/`, for example `exercises/05_vecs/vecs2.rs`; each topic also has a short `README.md`. Matching reference implementations live under `solutions/`. `Cargo.toml` registers every exercise and solution as a separate binary, while `.rustlings-state.txt` records local progress. Build artifacts belong in `target/` and must not be committed. There are no separate asset directories or integration-test tree.

## Build, Test, and Development Commands

- `rustlings` starts watch mode and advances through pending exercises.
- `rustlings run vecs2` compiles and tests one named exercise.
- `rustlings hint vecs2` displays guidance without changing the file.
- `cargo test --bin vecs2` runs the inline tests for one exercise directly.
- `cargo clippy --bin vecs2 --profile test` applies the lint configuration used by rust-analyzer.
- `cargo fmt --all -- --check` reports formatting differences; run `cargo fmt --all` to apply them.

Prefer targeted commands while exercises remain incomplete: `cargo test` or a workspace-wide Clippy run can fail on intentional placeholders in later exercises.

## Coding Style & Naming Conventions

Use standard `rustfmt` output and four-space indentation. Follow Rust naming conventions: `snake_case` for functions and variables, `CamelCase` for types and traits, and `SCREAMING_SNAKE_CASE` for constants. Keep solutions focused on the concept being taught; avoid unrelated refactors or extra dependencies. Do not introduce `unsafe` or unstable features, both of which are forbidden by `Cargo.toml`. Preserve exercise names such as `primitive_types4.rs` because Cargo binary names and Rustlings progress tracking depend on them.

## Testing Guidelines

Tests are colocated in each `.rs` file under `#[cfg(test)] mod tests` and use Rust's built-in test framework. Name tests descriptively with `snake_case`, typically `test_<behavior>`. Add assertions for normal behavior and relevant edge cases. Before submitting, run the changed exercise with `rustlings run <name>` and, when applicable, its paired solution with `cargo test --bin <name>_sol`.

## Commit & Pull Request Guidelines

Recent commits use imperative, sentence-style subjects such as `Update primitive_types state and solutions` and `Fix animal_habitat function`. Keep commits scoped to one exercise or topic and mention tests when they are materially changed. Pull requests should summarize the learning objective, list modified exercise/solution pairs, and report the exact validation commands run. Link related issues when available; screenshots are unnecessary for this CLI-only project. Avoid manually editing generated progress state unless the exercise completion requires it.
