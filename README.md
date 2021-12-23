# riscv-v-lite
Simplistic RISC-V emulator including Vector extension instructions

## The Emulator

The emulator is implemented in Rust using the Cargo package manager.
Two files are used for decoding and execution:
- [instrs.rs](rsim/src/instrs.rs) decodes raw instruction bits into pairs of (major opcode, instruction fields).
- [main.rs](rsim/src/main.rs) implements a Processor structure which executes instructions, as well as a Memory structure. 

To run the emulator on the example program:
1. Install Rust as directed by [rust-lang.org](https://www.rust-lang.org/tools/install)
2. Enter the [rsim/](/rsim/) directory
3. ```$ cargo run ../programs/build/mem.bin```

If an error occurs during execution, including any issues with unimplemented instructions, the processor state will be dumped and an error message will display.

Currently, the vector load/store instructions are not yet implemented, so an error will always occur.

## The Program

[The programs/ subfolder](/programs/) contains the source files and compilation output for a simple vectorized memcpy.

**You should not need to compile this program yourself - [programs/build/](/programs/build/) has all the artifacts you need**. 

This uses an LLVM-based toolchain for RISC-V, and assumes LLVM v13 is installed.
See [https://apt.llvm.org/](https://apt.llvm.org/) for instructions to download LLVM v13.

Required Packages:
- llvm-13
- clang-13
- lld-13
