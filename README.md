# 🔍 rusearch

A super-fast, parallel, recursive directory search CLI for developers and power users! 🚀

---

## ✨ Features

- ⚡ **Blazing Fast**: Uses parallelism (rayon) for super quick searches
- 📂 **Recursive**: Searches all files in a directory tree
- 🔎 **Flexible**: Search by pattern, pattern file, or a single shortcut argument
- 🦀 **Rust-Powered**: Built with Rust for reliability and speed
- 🖥️ **Simple CLI**: Easy to use, with clear error messages

---

## 🚀 Quick Start

### 1️⃣ Install (from source)

```sh
cargo build --release
```

The binary will be at `target/release/app` (rename to `rusearch` if you wish).

---

### 2️⃣ Usage

#### 🔥 Fastest Shortcut (search current directory recursively for a string):

```sh
./rusearch "pattern"
```

#### 🏷️ With options:

- Search for a pattern in a specific directory:
  ```sh
  ./rusearch --pattern "pattern" --dir /path/to/dir
  ```
- Search for a pattern from a file:
  ```sh
  ./rusearch --pattern-file pattern.txt --dir /path/to/dir
  ```

#### 🆚 All CLI Options

| Option            | Description                                              |
|-------------------|----------------------------------------------------------|
| `--pattern, -p`   | Pattern to search for (substring)                        |
| `--pattern-file`  | File containing pattern to search for                    |
| `--dir, -d`       | Directory to search (default: current directory)         |
| `"pattern"`       | (Positional) Shortcut: search for this pattern in `.`    |

> ⚠️ Only one of `--pattern`, `--pattern-file`, or positional `"pattern"` may be provided at a time.

---

### 🧑‍💻 Examples

- Search for "TODO" in the current directory:
  ```sh
  ./rusearch "TODO"
  ```
- Search for "main" in `/src`:
  ```sh
  ./rusearch --pattern main --dir ./src
  ```
- Search for a pattern from a file:
  ```sh
  ./rusearch --pattern-file mypattern.txt
  ```

---

## 🛠️ Build & Develop

- Build: `cargo build`
- Run: `cargo run -- [args]`
- Format: `cargo fmt`
- Lint: `cargo check`
- Test: `cargo test` (no tests yet)

---

## 📝 Output Format

Each match prints:

```
<file_path>:<line_number>:<line>
```

---

## ❓ FAQ

- **Q: Can I use both --pattern and --pattern-file?**
  - ❌ No, only one pattern source at a time.
- **Q: What files are searched?**
  - All regular files, recursively, from the specified directory.
- **Q: Is the search case-sensitive?**
  - ✅ Yes (by default).

---

## 🤝 Contributing

PRs welcome! Please run `cargo fmt` before submitting.

---

## 📄 License

MIT/Apache-2.0

---

## 🦀 Made with Rust and ❤️
