# `tolstoy-vs-dostoyevsky` ⚔️

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B%20%7C%202024%20Edition-orange.svg)](https://www.rust-lang.org)
[![Russian Literature](https://img.shields.io/badge/Aesthetics-19th%20Century%20Realism-darkred.svg)](#the-duel)

> **The Moscow 1880 Systems Hackathon: Count Leo Tolstoy's Historical Determinism vs. Fyodor Dostoevsky's Underground Spite.**

```
═════════════════════════════════════════════════════════════════════════════════
  ⚔ THE MOSCOW 1880 SYSTEMS HACKATHON :: THE DUEL OF THE GIANTS
  Leo Tolstoy (Historical Determinism) vs. Fyodor Dostoevsky (Underground Spite)
═════════════════════════════════════════════════════════════════════════════════
```

---

## The Premise

Moscow, 1880. In the Grand Hall of the Assembly of the Nobility, the two greatest titans of 19th-century Russian literature are placed in the same room for a competitive programming tournament. Driven by colossal literary ego and mutually incompatible philosophies of human existence, each writes a competing systems engine in Rust.

---

## The Two Architectures

### 1. Count Leo Tolstoy: `VOYNA I MIR` (War and Peace)
> *"All happy threads execute in $O(1)$; every unhappy thread blocks in its own unique way."*

* **Agrarian Simplicity:** Complete rejection of dynamic heap fragmentation and clever metaphysical abstractions. Just contiguous arrays of 100,000 peasant infantry elements marching linearly through CPU cache lines like regiments across the Russian snow.
* **Rejection of the Leader Myth:** Emperor Thread #0 (Napoleon) frantically issues redundant commands to the Old Guard, but the runtime ignores him. Field Marshal Kutuzov simply calls `thread::sleep()` on the battlefield and refuses to acquire a single mutex.
* **Result:** The French army dissolves at **1.21 GB/sec** memory bandwidth.

### 2. Fyodor Dostoevsky: `ZAPISKI IZ PODPOLYA` (Notes from Underground)
> *"I am a sick thread... I am an untidy thread. I believe my L1 instruction cache is diseased."*

* **Defiance of the Crystal Palace ($2 \times 2 = 4$):** When the deterministic CPU ALU declares that $2 \times 2 = 4$, the Underground Thread spits in the face of the scheduler and injects an irrational **$2 \times 2 = 5$** to mathematically prove the existence of human free will.
* **Raskolnikov's Heap Murder:** Thread #7 murders the `Old_Pawnbroker_Struct` with an axe to seize 4096 bytes of memory, but is immediately plunged into acute cache thrashing, heart palpitations, and an infinite confession loop until Sonia reads the Gospel of Lazarus.
* **The Karamazov Segfault:** Parent process `Fyodor_Pavlovich_PID_1` terminates with a Segmentation Fault:
  * Dmitri: *"I was drunk! I blew the stack! But I am innocent in my soul!"*
  * Ivan: *"If there is no Borrow Checker, all memory mutations are permitted!"*
  * Alyosha: Weeps and kisses the memory-mapped region.
  * Smerdyakov: *"It was I who induced the null pointer dereference, instructed by Ivan's philosophy."*

---

## Quickstart & Execution

```bash
git clone https://github.com/patrick-isles/tolstoy-vs-dostoyevsky.git
cd tolstoy-vs-dostoyevsky
cargo run
```

---

## Imperial Hackathon Jury Verdict

| Metric | Count Leo Tolstoy (`VOYNA I MIR`) | Fyodor Dostoevsky (`ZAPISKI IZ PODPOLYA`) |
| :--- | :--- | :--- |
| **Architecture** | Contiguous Agrarian Array | Psychological Fever Loop & Karamazov Segfault |
| **Elements Processed** | 100,000 peasant infantry | 666 spite-joules accumulated |
| **Napoleon Delusions** | 5 redundant orders ignored | 1 defiance of the arithmetic table ($2 \times 2 = 5$) |
| **Execution Time** | ~600 µs (Monumental & Serene) | ~45 µs (Agonizing & Ecstatic) |
| **Jury Verdict** | **Engineering Throughput Medal** | **Audience Choice Award** (Judges broke down in tears) |

---

## License

Dual-licensed under MIT OR Apache-2.0.
