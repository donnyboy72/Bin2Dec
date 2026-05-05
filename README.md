# Binary to Decimal Converter in Rust

A simple, robust command-line tool that takes an 8-bit binary input and converts it into its decimal (base-10) equivalent.

## 🚀 How it Works
The program prompts the user for exactly 8 binary digits. It performs strict validation to ensure:
- The input is exactly 8 characters long.
- The input contains only `1`s and `0`s.
- The program loops until a valid input is provided.

## 🧠 What I Learned
While the logic of binary conversion was familiar, implementing this in Rust taught me several core concepts of the language:

### 1. String Handling & Memory
I learned the difference between `String` (owned data) and `&str` (string slices). Dealing with `.trim()` taught me how Rust handles whitespace and newlines from `stdin`.

### 2. Type Strictness (Integer Overflow)
I encountered a "subtract with overflow" panic. This taught me that:
- Rust is extremely strict about unsigned integers (`u32`).
- `u32` cannot be negative, so `0 - 1` triggers a safety panic.
- Power functions like `.pow()` require specific types (the exponent must be `u32`).

### 3. Iterators and Ownership
I explored different ways to traverse data in Rust:
- Using `for` loops with ranges.
- Converting strings to character iterators with `.chars()`.
- Understanding why calling `.to_string()` inside a loop can be inefficient compared to using references.

### 4. Input Validation & Control Flow
I used `loop` and `match` to create a "retry" mechanism, ensuring the program doesn't crash on bad user input—a key pattern in systems programming.

## 🛠️ Requirements
- Rust (Cargo) installed.

## 🏃 Usage
1. Clone the repo.
2. Run `cargo run`.
3. Enter an 8-digit binary number (e.g., `10101010`).
