# riscv-v-lite
Simplistic RISC-V emulator including Vector extension instructions

## The Emulator

The emulator is implemented in Rust using the Cargo package manager.
It is split across multiple files:
- [src/memory.rs](/rsim/src/memory.rs) defines a Memory struct.
- [src/processor/](/rsim/src/processor/) holds actual processor files:
    - [decode.rs](rsim/src/processor/decode.rs) decodes raw instruction bits into pairs of (major opcode, instruction fields).
    - [vector.rs](rsim/src/processor/vector.rs) implements a separate vector unit, which holds and manipulates all vector state.
    - [mod.rs](rsim/src/processor/mod.rs) implements the scalar processor, which calls out to a vector unit on-demand.

To run the emulator on the example program:
1. Install Rust as directed by [rust-lang.org](https://www.rust-lang.org/tools/install)
2. Enter the [rsim/](/rsim/) directory
3. ```$ cargo run ../programs/build/mem.bin```

Further documentation can be also be generated:

```$ cargo doc --open```

And some tests embedded in said documentation can be run:

```$ cargo test```

If an error occurs during execution, including any issues with unimplemented instructions, the processor state will be dumped and an error message will display.

All instructions that occur in the example program are now implemented, so the program should succeed.
This is signified by writing 1 (or 0 if the program failed) to the address `0xF000_0000`.
When this happens, the Memory module will throw an ""error"" to report the success.

```
Error: Failed to execute decoded instruction Store SType { funct3: 2, rs1: 11, rs2: 10, imm: 0 }

Caused by:
    Program returned a value: 1
```

As the program has returned 1, it has been successful! 🎉

## The Program

[The programs/ subfolder](/programs/) contains the source files and compilation output for a simple vectorized memcpy.

Currently the vectorized memcpy tests
- Behaviour with LMUL=8, SEW=32
- Unmasked simple unit vector loads,stores
- Behaviour when the application vector length is not a multiple of elements per register group - i.e. behaviour for vector loads/stores with a tail

It does NOT test (and thus the emulator doesn't support)
- Unit WholeRegister, ByteMaskLoad, or FaultOnlyFirst loads
- Unit WholeRegister or ByteMaskStore stores
- Strided or Indexed loads/stores
- Segmented loads/stores
- Masked loads/stores
- Fractional or 1,2,4 LMUL
- SEW=8,16 (64 is not supported because it's a 32-bit program)
- Any changes to `vstart`
- Any accesses to CSRs e.g. `vtype`
- Any arithmetic

**You should not need to compile this program yourself - [programs/build/](/programs/build/) has all the artifacts you need**. 

This uses an LLVM-based toolchain for RISC-V, and assumes LLVM v13 is installed.
See [https://apt.llvm.org/](https://apt.llvm.org/) for instructions to download LLVM v13.

Required Packages:
- llvm-13
- clang-13
- lld-13
