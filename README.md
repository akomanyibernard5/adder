# Adder Compiler

A compiler for the Adder language, built for a compilers course.

## Description

This compiler translates programs written in the Adder language into x86-64 assembly code. The Adder language supports basic arithmetic operations on 32-bit signed integers.

## Dependencies

- Rust (with Cargo)
- NASM (Netwide Assembler)
- GCC/Clang (for linking)

## Building

```bash
cargo build --release
```

## Running

To compile an Adder program:

```bash
cargo run -- test/foo.snek test/foo.s
```

To build and run a complete test:

```bash
make test/foo.run
./test/foo.run
```

## Language Features

The Adder language supports:
- Integer literals (32-bit signed)
- `add1`: increment by 1
- `sub1`: decrement by 1
- `negate`: negate a number

## Examples

```
37                          # Returns 37
(add1 5)                    # Returns 6
(sub1 10)                   # Returns 9
(negate 7)                  # Returns -7
(add1 (add1 5))            # Returns 7
(negate (negate (negate 2))) # Returns -2
```
