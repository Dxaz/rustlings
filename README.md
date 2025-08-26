
# My Rustlings 🦀

**Rustlings** provides a collection of small, guided exercises designed to help you get hands-on experience reading and writing Rust code—and learn how to respond to compiler messages.

## Purpose

* Reinforce concepts from "The Book" through practical, hands-on challenges.
* Enhances my debugging skills by fixing compiler errors directly.

---

## Prerequisites ✅

* `rustup` & `cargo`.
---

## Installation 📦

```bash
cargo install rustlings
```

```bash
rustlings init
```

---

## Using Rustlings ⚙️

### Watch Mode (recommended) 👀

```bash
rustlings
```


### Verify Once 1️⃣

* To check all exercises without watch-mode:

```bash
rustlings check-all
```

* To run a specific exercise:

```bash
rustlings run <exercise_name>
```

* Manually get hints:

```bash
rustlings hint <exercise_name>
```

---

## Exercise–Chapter Map 📖


| Exercise Topic   | Corresponding Chapter         |
| ---------------- | ----------------------------- |
| variables        | §3.1                          |
| functions        | §3.3                          |
| if               | §3.5                          |
| primitive\_types | §3.2, §4.3                    |
| vecs             | §8.1                          |
| move\_semantics  | §4.1–4.2                      |
| structs          | §5.1, §5.3                    |
| enums            | §6, §18.3                     |
| strings          | §8.2                          |
| modules          | §7                            |
| hashmaps         | §8.3                          |
| options          | §10.1                         |
| error\_handling  | §9                            |
| generics         | §10                           |
| traits           | §10.2                         |
| tests            | §11.1                         |
| lifetimes        | §10.3                         |
| iterators        | §13.2–13.4                    |
| threads          | §16.1–16.3                    |
| smart\_pointers  | §15, §16.3                    |
| macros           | §19.6                         |
| clippy           | §21.4                         |
| conversions      | – (no direct chapter mapping) |


---

## Uninstall ❌

```bash
rm -rf rustlings
cargo uninstall rustlings
```

---
