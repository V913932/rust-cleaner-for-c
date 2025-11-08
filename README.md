# libcleaner.so

**A garbage cleaner written in Rust to clean the Garbages on C.**

`libcleaner.so` is a lightweight Rust library designed to safely and efficiently clean up dynamically allocated memory from C programs. It bridges the raw power of C with the safety and ownership model of Rust, allowing C developers to offload memory cleanup to a Rust guardian.

---

## 🧠 Features

- 🔗 **FFI-compatible**: Easily callable from C via `extern "C"` interface
- 🧹 **Safe memory cleanup**: Uses Rust's ownership model to free C-allocated strings
- 🛡️ **Drop-based deallocation**: Automatically drops `CString` from raw pointers
- 🧪 **Minimal overhead**: Pure Rust, no runtime dependencies
- 🧙‍♂️ **Ceremonial integration**: Designed for projects that ritualize memory management

---

## 📦 Usage

### 1. Link the library in your C project:

```c
extern void clean_garbage(char*);
generate garbaage:
char* g = malloc(100);
strcpy(g, "This is garbage from C.");
clean it with this lib:
clean_garbage(g);

if you want to make it a static libary,go to cargo toml and change the lib from Cargo.toml to this:```toml
[lib]
crate-type = ["staticlib"]
