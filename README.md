
# `password-gen`

**🔐 A simple, interactive password generator built with Rust.**

Generate secure passwords right from your terminal — with options for length, symbols, numbers, and more. Supports both CLI flags and interactive mode for a smooth experience.

---

### Features

- ⚙️ Customizable length and character sets
- 💬 Interactive mode (guided prompts)
- 🧠 Clipboard support (optional)
- 🦀 Fast, safe, and built with Rust

---

### Usage

**With CLI flags:**

```bash
password-gen --length 16 --numbers --symbols --copy
```

**Interactive mode:**

```bash
password-gen --interactive
```

---

### 📥 Install

Clone & run:

```bash
git clone https://github.com/Isaiah38/Password-gen-rust.git
cd password-gen
cargo run -- --interactive
```