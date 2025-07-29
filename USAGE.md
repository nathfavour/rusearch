# 📖 Usage Guide for rusearch

## 🚀 Quick Usage

### 🔥 Fastest Shortcut

Search the current directory recursively for a string:

```sh
./rusearch "pattern"
```

---

### 🏷️ With Options

- Search for a pattern in a specific directory:
  ```sh
  ./rusearch --pattern "pattern" --dir /path/to/dir
  ```
- Search for a pattern from a file:
  ```sh
  ./rusearch --pattern-file pattern.txt --dir /path/to/dir
  ```

---

## 🆚 CLI Options Table

| Option            | Description                                              |
|-------------------|----------------------------------------------------------|
| `--pattern, -p`   | Pattern to search for (substring)                        |
| `--pattern-file`  | File containing pattern to search for                    |
| `--dir, -d`       | Directory to search (default: current directory)         |
| `"pattern"`       | (Positional) Shortcut: search for this pattern in `.`    |

> ⚠️ Only one of `--pattern`, `--pattern-file`, or positional `"pattern"` may be provided at a time.

---

## 🧑‍💻 Examples

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
