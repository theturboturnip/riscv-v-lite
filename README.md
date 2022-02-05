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
The result of the program is written to the address `0xF000_0000`.
When this happens, the Memory module will throw an ""error"" to report the success.

```
Error: Failed to execute decoded instruction Store SType { funct3: 2, rs1: 11, rs2: 10, imm: 0 }

Caused by:
    Program returned a value = 0x3FFF (expected 0x3FFF) = 0b0011111111111111
```

As the program has returned the expected value, it has been successful! 🎉

The bits of the output represent the outcomes of various tests - see [programs/vector_memcpy.c](/programs/vector_memcpy.c) to tell which bit corresponds to which test.

## The Program

[The programs/ subfolder](/programs/) contains the source files and compilation output for a simple vectorized memcpy.

Currently the vectorized memcpy tests
- Behaviour with LMUL={1/2, 1, 2, 4, 8}, SEW=32
  - (LMUL={1/4, 1/8} not supported by intrinsics for 32-bit elements)
- Behaviour with LMUL=8, SEW={8,16}
- Unmasked AND Masked Unit vector loads,stores (SEW=32)
  - Limited vector arithmetic required to generate masks
- Unmasked Strided vector loads,stores (SEW=8,16,32)
- Unmasked Indexed vector loads,stores (SEW=32)
- Unmasked Segmented vector loads,stores (SEW=32)
- Unit ByteMask loads, stores
- Unit FaultOnlyFirst loads (SEW=32, LMUL=1)
- Unit WholeRegister accesses (SEW=32, LMUL=1)
- Behaviour when the application vector length is not a multiple of elements per register group - i.e. behaviour for vector loads/stores with a tail
- Accesses to CSRs e.g. `vl`

It does NOT test (and thus the emulator doesn't necessarily support)
- Any changes to `vstart`
- Most arithmetic

**You should not need to compile this program yourself - [programs/build/](/programs/build/) has all the artifacts you need**. 

This uses an LLVM-based toolchain for RISC-V, and assumes LLVM v13 is installed.
See [https://apt.llvm.org/](https://apt.llvm.org/) for instructions to download LLVM v13.

Required Packages:
- llvm-13
- clang-13
- lld-13
