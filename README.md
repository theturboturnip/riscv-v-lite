# riscv-v-lite
Simplistic RISC-V emulator including Vector extension instructions.

Supports `rv32imv`, `rv64im`, `rv64imxcheri`, and `rv64imxcheri-int` architectures.

## The Emulator

The emulator is implemented in Rust using the Cargo package manager.
It is split across multiple modules:
- [crate::processor](/rsim/src/processor/) holds actual processor files:
  - [::elements](/rsim/src/processor/elements.rs) holds processor "elements": register files and memory
    - [::cheri](/rsim/src/processor/elements/cheri.rs) holds CHERI-ready processor "elements", and is the source for capability types used throughout the emulator.
  - [::isa_mods](/rsim/src/processor/isa_mods.rs) holds different ISA modules that can be combined to build a processor
  - [::models](/rsim/src/processor/models.rs) holds different processor models that combine various ISA modules:
    - [::rv32imv](rsim/src/processor/models/rv32imv.rs) defines a 32-bit processor supporting Integer,Multiply,Vector,CSR exts
    - [::rv64im](rsim/src/processor/models/rv64im.rs) defines a 64-bit processor supporting Integer,Multiply,CSR exts
    - [::rv64imxcheri](rsim/src/processor/models/rv64imxcheri.rs) defines a 64-bit CHERI processor supporting Integer,Multiply,CSR exts
  - [::decode](/rsim/src/processor/decode.rs) exposes logic for decoding 32-bit instructions (Compressed instructions aren't supported yet)
  - [::exceptions](/rsim/src/processor/exceptions.rs) defines all Memory, Instruction, and Capability errors that the processor can throw and trap.

To run the emulator on the example program:
1. Install Rust as directed by [rust-lang.org](https://www.rust-lang.org/tools/install)
2. Enter the [rsim/](/rsim/) directory
3. ```$ cargo run direct rv32imv ../programs/build/llvm-13-rv32imv/vector_memcpy/mem.bin```
4. Running CHERI requires the actual .elf of the program:
5. ```$ cargo run direct rv64imxcheri ../programs/build/llvm-13-rv64imxcheri/hello_world/hello_world.elf```

Further documentation can be also be generated:

```$ cargo doc --open```

And some tests embedded in said documentation can be run:

```$ cargo test```

If an error occurs during execution, including any issues with unimplemented instructions, the processor state will be dumped and an error message will display.

Run `./test.sh` to build and test the emulator, compile the test programs if necessary, and run all tests.

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

### Compiling the program

`$ cd ./programs/ && make`

The top-level Makefile in [programs/](/programs/) will invoke CMake to build the program with each toolchain file.
Toolchains are specified in CMake files e.g. `programs/TC-gcc-rv32iv.cmake`.

Currently, three toolchains are supported:
- `gcc`, which use a custom GNU toolchain for 'rv32iv' that includes vector intrinsics
  - This uses the riscv gcc toolchain from the path, may change later to use a specific version/environment variable
- `llvm-13-...xcheri` uses the Cheri toolchain installed in `~/cheri/`.
  - Compiling vector code currently requires my fork, [theturboturnip/llvm-project](https://github.com/theturboturnip/llvm-project)
- `llvm-13`, which uses LLVM v13
  - Make sure LLVM v13 tools (e.g. `clang-13`) are on your PATH
- `llvm-trunk`, which uses a custom LLVM build
  - Make sure to set the path to your LLVM bin directory in the toolchain file `TC-llvm-trunk-rv32iv.cmake`.

See [https://apt.llvm.org/](https://apt.llvm.org/) for instructions to download LLVM v13.
Required Packages:
- llvm-13
- clang-13
- lld-13

I used [https://github.com/riscv-collab/riscv-gnu-toolchain/tree/rvv-intrinsic](https://github.com/riscv-collab/riscv-gnu-toolchain/tree/rvv-intrinsic) as my base for building GNU toolchain.
This had a few issues - the largest was that the `riscv-glibc` submodule has moved from GitHub to [Sourceware](https://sourceware.org/git/?p=glibc.git), and even then apparently that repository's 2.29 branch doesn't have the correct patches to work with RV32.
Thankfully, you don't need `glibc` to work.

GCC's version is very incomplete - `vector_memcpy.c` has to disable some test cases for it to compile.
