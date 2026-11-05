# 🦀 rpl: Rust Programming Language

| Stage | Material                                                                  | Status |
| :---: | ------------------------------------------------------------------------- | :----: |
|   1   | _The Rust Programming Language_: prerequisites (ch 1 to 11, 13.1 to 13.2) |   ✅   |
|   2   | _The Secrets of Rust: Tools_                                              |   ⏳   |
|   3   | _The Rust Programming Language_: advanced chapters (ch 15 to 20)          |   💤   |
|   4   | _Rust for Rustaceans_                                                     |   💤   |

---

## 📁 Repo layout

```text
rpl
├── Cargo.toml        # workspace root
├── book/             # Rust Book exercises, one crate per section (e.g. 4_2_borrowing)
├── secrets/          # The Secrets of Rust: Tools, one crate per project
├── practice/
└── rustlings/
```

```toml
[workspace]
members = ["book/*", "secrets/*", "practice/*", "rustlings"]
resolver = "3"
```

Run any crate with `cargo run` from inside its folder, or `cargo run -p <package-name>` from the root.

---

## 1️⃣ Rust Book: prerequisites

[The Rust Programming Language](https://rust-lang.github.io/book/)

- [x] **1 to 10**: getting started → generics, traits, lifetimes
- [x] **11 Writing Automated Tests**
  - [x] 11.1 How to Write Tests
  - [x] 11.2 Controlling How Tests Are Run
  - [x] 11.3 Test Organization
- [x] **13.1** Closures: Anonymous Functions that Capture Their Environment
- [x] **13.2** Processing a Series of Items with Iterators

---

## 2️⃣ The Secrets of Rust: Tools

[Book page](https://bitfieldconsulting.com/books/rust-tools) · [Companion repo](https://github.com/bitfield/tsr-tools)

Project-based. Each project becomes a crate under `secrets/` (e.g. `secrets/ch_03_args`).

- [ ] **Ch 1 to 4**: crates, errors, arguments, flags (`clap`, `assert_cmd`) ← **_current_**
- [ ] **Ch 5 to 6**: files, `Path`/`AsRef`, documentation, publishing, `serde` and data modelling
- [ ] **Ch 7**: HTTP client, mock testing, the newtype pattern
- [ ] **Ch 8**: running commands, and a Cargo plugin
- [ ] Exercises done

---

## 3️⃣ Rust Book: advanced

Knowledge _Rust for Rustaceans_ assumes.

- [ ] **15 Smart Pointers** (`read`, all of it)
  - [ ] 15.1 `Box<T>` · 15.2 `Deref` · 15.3 `Drop`
  - [ ] 15.4 `Rc<T>` · 15.5 `RefCell<T>` and interior mutability · 15.6 Reference cycles
- [ ] **16 Fearless Concurrency**
- [ ] **17 Async and Await**
- [ ] **18.1** Characteristics of OO languages
- [ ] **18.2** Trait objects
- [ ] **19 Patterns and Matching**
- [ ] **20.1** Unsafe Rust
- [ ] **20.2** Advanced Traits
- [ ] **20.3** Advanced Types
- [ ] **20.4** Advanced Functions and Closures
- [ ] **20.5** Macros

---

## 4️⃣ Rust for Rustaceans

[Book page](https://nostarch.com/rust-rustaceans)

- [ ] **1** Foundations
- [ ] **2** Types
- [ ] **3** Designing Interfaces
- [ ] **4** Error Handling
- [ ] **5** Project Structure
- [ ] **6** Testing
- [ ] **7** Macros
- [ ] **8** Asynchronous Programming
- [ ] **9** Unsafe Code
- [ ] **10** Concurrency (and Parallelism)
- [ ] **11** Foreign Function Interfaces
- [ ] **12** Rust Without the Standard Library
- [ ] **13** The Rust Ecosystem (read last)

---

## ⏭️ Deferred (revisit as reference)

- [ ] Rust Book **12** An I/O Project (Secrets covers the same ground)
- [ ] Rust Book **13.3** Improving Our I/O Project · **13.4** Loops vs. Iterators
- [ ] Rust Book **14** More about Cargo and Crates.io
- [ ] Rust Book **18.3** OO design pattern · **21** Multithreaded web server
