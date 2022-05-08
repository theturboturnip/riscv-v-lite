#!/usr/bin/env bash

(cd ../programs/ && make) || exit 1

cargo build || exit 1

cargo run direct rv32imv ../programs/build/llvm-13-rv32imv/vector_memcpy_old/mem.bin
cargo run direct rv32imv ../programs/build/llvm-13-rv32imv/vector_memcpy/mem.bin
cargo run direct rv32imv ../programs/build/llvm-13-rv32imv/hello_world/mem.bin
cargo run direct rv32imv ../programs/build/llvm-13-rv32imv/vector_memcpy_pointers/mem.bin

cargo run direct rv64imv ../programs/build/llvm-13-rv64imv/vector_memcpy_old/mem.bin
cargo run direct rv64imv ../programs/build/llvm-13-rv64imv/vector_memcpy/mem.bin
cargo run direct rv64imv ../programs/build/llvm-13-rv64imv/hello_world/mem.bin
cargo run direct rv64imv ../programs/build/llvm-13-rv64imv/vector_memcpy_pointers/mem.bin

# cargo run direct rv64imvxcheri ../programs/build/llvm-13-rv64imxcheri/vector_memcpy/vector_memcpy.elf
# cargo run direct rv64imvxcheri ../programs/build/llvm-13-rv64imxcheri/hello_world/hello_world.elf
cargo run direct rv64imvxcheri ../programs/build/llvm-13-rv64imxcheri/vector_memcpy_pointers/vector_memcpy_pointers.elf

# cargo run direct rv64imvxcheri-int ../programs/build/llvm-13-rv64imxcheri-int/vector_memcpy/vector_memcpy.elf
# cargo run direct rv64imvxcheri-int ../programs/build/llvm-13-rv64imxcheri-int/hello_world/hello_world.elf
cargo run direct rv64imvxcheri-int ../programs/build/llvm-13-rv64imxcheri-int/vector_memcpy_pointers/vector_memcpy_pointers.elf
