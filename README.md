<div align="center">

```
        ⚙️  ⚙️  ⚙️  ⚙️  ⚙️  ⚙️  ⚙️  ⚙️  ⚙️  ⚙️
      ██████╗ ██╗   ██╗███████╗████████╗
      ██╔══██╗██║   ██║██╔════╝╚══██╔══╝
      ██████╔╝██║   ██║███████╗   ██║
      ██╔══██╗██║   ██║╚════██║   ██║
      ██║  ██║╚██████╔╝███████║   ██║
      ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝
        ⚙️  ⚙️  ⚙️  ⚙️  ⚙️  ⚙️  ⚙️  ⚙️  ⚙️  ⚙️
```

# 🦀 THE RUST ENGINE ROOM 🦀
### *A learner's workshop — bolting Rust concepts together, one gear at a time*

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/STATUS-UNDER%20CONSTRUCTION-orange?style=for-the-badge&logo=gear&logoColor=white)](#)
[![Level](https://img.shields.io/badge/LEVEL-BEGINNER-CE422B?style=for-the-badge&logo=levelsdotfyi&logoColor=white)](#)


</div>

<br>

```
╔══════════════════════════════════════════════════════════════╗
║  ⚠️  WORKSHOP NOTICE: Fresh gears being tested. Handle with   ║
║      curiosity. Bugs may fly. Compiler errors are welcome.    ║
╚══════════════════════════════════════════════════════════════╝
```

## 🧰 What Is This Place?

This repo is my personal **machine shop** for learning Rust 🦀 — every file is a station where I bolt on a new concept, test it, and leave notes on how the gear turns. No prior Rust experience, just curiosity and a compiler that yells at me until I get it right.

<br>

## ⚙️ Gearbox — Topics Assembled So Far

<div align="center">

| Gear # | Component | Status |
|:---:|:---|:---:|
| 01 | `println!` / `print!` macros | ✅ |
| 02 | String formatting with `{}` placeholders | ✅ |
| 03 | Multiple & ordered placeholders | ✅ |
| 04 | Variables — immutability by default | ✅ |
| 05 | Mutable variables (`mut`) | ✅ |
| 06 | Type inference vs explicit typing | ✅ |
| 07 | Core data types — `i32`, `f64`, `&str`, `bool`, `char` | ✅ |
| 08 | Constants (`const`) | ✅ |
| 09 | Logical operators — `&&` `\|\|` `!` | ✅ |
| 10 | `if / else` statements | ✅ |
| 11 | `if / else` as an **expression** | ✅ |
| 12 | `match` statement (switch-like control flow) | ✅ |

</div>

<br>

## 🔧 Sample From the Shop Floor

A peek at the kind of machinery being built inside `main.rs`:

```rust
// Gear 12 — pattern matching, Rust-style
let day = 4;
match day {
    1 => println!("Monday"),
    2 => println!("Tuesday"),
    3 => println!("Wednesday"),
    4 => println!("Thursday"),
    5 => println!("Friday"),
    6 => println!("Saturday"),
    7 => println!("Sunday"),
    _ => println!("Invalid day"), // catch-all gear
}
```

```rust
// Gear 06 — mutability, the safety lever
let mut x = 5;
println!("Before: {}", x);
x = 10; // valid only because `mut` unlocked the lever
println!("After: {}", x);
```

<br>

## 🚀 Firing Up the Engine

```bash
# Clone the workshop
git clone https://github.com/Aashutosh31/my_first_rust_project_basics.git
cd my_first_rust_project_basics

# Run the machine
cargo run
```

<br>

## 🗺️ Blueprint — What's Next on the Bench

```
   [x] Variables & Data Types      [x] Control Flow (if/match)
   [ ] Ownership & Borrowing       [ ] Structs & Enums
   [ ] Vectors & Collections       [ ] Error Handling (Result/Option)
   [ ] Functions & Closures        [ ] Traits & Generics
   [ ] Modules & Crates            [ ] Lifetimes
```

<br>

## 📁 Workshop Layout

```
.
└── main.rs   → the primary workbench where each concept is welded together
```

<br>

<div align="center">

```
   ⚙️  built gear by gear, bug by bug, coffee by coffee ⚙️
```

**🦀 Made while learning Rust — one `cargo run` at a time 🦀**

</div>