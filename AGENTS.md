# AGENTS.md

## Build, Lint, and Test

- **Build:**  
  `cargo build`
- **Run:**  
  `cargo run -- [args]`
- **Format (rustfmt):**  
  `cargo fmt` (check: `cargo fmt -- --check`)
- **Lint:**  
  *No clippy installed; use `cargo check` for basic linting.*
- **Test:**  
  `cargo test`  
  *No tests currently implemented. Add tests in `tests/` or as `#[cfg(test)]` modules in source files.*

## Code Style Guidelines

- **Imports:**  
  - Group standard, external, and internal imports separately.
  - Use absolute paths for external crates.
- **Formatting:**  
  - Enforced by `rustfmt`. Run `cargo fmt` before committing.
- **Types:**  
  - Prefer explicit types for function signatures and struct fields.
  - Use `Result<T, E>` for fallible operations.
- **Naming Conventions:**  
  - Use `snake_case` for variables, functions, and modules.
  - Use `CamelCase` for types and structs.
  - Constants in `SCREAMING_SNAKE_CASE`.
- **Error Handling:**  
  - Use `Result` and `Option` idiomatically.
  - Print errors to `stderr` and exit with nonzero code on fatal errors.
- **Arguments:**  
  - Use `clap` for CLI parsing (`Args` struct in `args.rs`).
  - Only one of `--pattern` or `--pattern-file` may be provided.
- **Parallelism:**  
  - Use `rayon` for parallel file processing.
- **File Structure:**  
  - Main logic in `src/`, entrypoint in `main.rs`.
  - Add tests in `#[cfg(test)]` modules or `tests/` directory.

note:

always run git add ., generate a single concise sentence commit message without any author info or new line characters, then push. do all these after any change.