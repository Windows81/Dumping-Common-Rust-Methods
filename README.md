# Dumping Common Rust Methods

Consult [`./results/`](./results/) for first-byte dumps of commonly used Rust functions. When reading hex, look out for any `C3` bytes, as these indicate `ret` (return) instructions in x86.

**Especially the one that you may want to trace with a debugger such as x64dbg.**

I did not use LLMs for any component of this project as of 2026-08-14.

## How?

The program at [`./main.rs`](./main.rs) iterates through a bunch of functions:

1. takes a pointer to each function, then
2. dumps the first 72 bytes _in memory_ after the pointer location, and
3. scans the _entire_ binary content of the running executable for regions which match a small portion of these 72 bytes.

To attempt completeness of scope, I use GitHub Actions (link to [workflow file](./.github/workflows/main.yml)) to compile those methods on various [target triples](https://doc.rust-lang.org/cargo/commands/cargo-rustc.html#option-cargo-rustc---target) for Windows and GNU/Linux; both are modern x86-based systems.

To reproduce a runner action on your target, execute:

```sh
#                {target}              {dep}   {min}  {max}
python action.py x86_64-pc-windows-gnu reqwest 0.12.0 9007199254740991.9007199254740991.9007199254740991
```

**A full list of dumped functions can be derived from reading [`./main.rs`](./main.rs).**

## Calling Convention?

This [experiment from December 2021](https://github.com/phip1611/rust-different-calling-conventions-example) shows that Rust allows you to manually define what calling convention to use.

However, from what I can gather, the defaults for a _hypothetical_ `add(a: i64, b: i64) -> i64` are:

| Parameter | Register (Unix-based) | Register (Windows) |
| --------- | --------------------- | ------------------ |
| `a`       | `rdi`                 | `rcx`              |
| `b`       | `rsi`                 | `rdx`              |
| (return)  | `rax`                 | `rax`              |
