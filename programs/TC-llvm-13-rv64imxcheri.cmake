# usage
# cmake -DCMAKE_TOOLCHAIN_FILE=../TC-gcc-rv32i.cmake ../

# The Generic system name is used for embedded targets (targets without OS) in
# CMake
set( CMAKE_SYSTEM_NAME          Generic )
set( CMAKE_SYSTEM_PROCESSOR     rv64imv0p10xcheri   ) #v0p10
set( CMAKE_EXECUTABLE_SUFFIX    ".elf" )
set( CMAKE_EXECUTABLE_SUFFIX_C    ".elf" )

# Look for RISC-V CLANG
SET(RISCV_CLANG_COMPILER_MAYBE "$ENV{HOME}/cheri/output/sdk/bin/clang")

# Select which is found
if (EXISTS ${RISCV_CLANG_COMPILER_MAYBE})
set( RISCV_CLANG_COMPILER ${RISCV_CLANG_COMPILER_MAYBE})
else()
message(FATAL_ERROR "RISC-V CHERI CLANG-13 not found. ${RISCV_CLANG_COMPILER_MAYBE}")
endif()

message( "RISC-V CLANG-13 found: ${RISCV_CLANG_COMPILER}")

get_filename_component(RISCV_TOOLCHAIN_BIN_PATH     ${RISCV_CLANG_COMPILER} DIRECTORY)
get_filename_component(RISCV_TOOLCHAIN_BIN_CLANG    ${RISCV_CLANG_COMPILER} NAME_WE)
get_filename_component(RISCV_TOOLCHAIN_BIN_EXT      ${RISCV_CLANG_COMPILER} EXT)
set(RISCV_TOOLCHAIN_BIN_SUFFIX "")

message( "RISC-V CHERI LLVM-13 Path: ${RISCV_TOOLCHAIN_BIN_PATH}" )

# where is the target environment located
set(CMAKE_FIND_ROOT_PATH ${RISCV_TOOLCHAIN_BIN_PATH}/../)

# adjust the default behavior of the FIND_XXX() commands:
# search programs in the host environment
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
# search headers and libraries in the target environment
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)

# specify the cross compiler. We force the compiler so that CMake doesn't
# attempt to build a simple test program as this will fail without us using
# the -nostartfiles option on the command line

set(CMAKE_ASM_COMPILER ${RISCV_TOOLCHAIN_BIN_PATH}/clang${RISCV_TOOLCHAIN_BIN_SUFFIX})
set(CMAKE_AR           ${RISCV_TOOLCHAIN_BIN_PATH}/llvm-ar${RISCV_TOOLCHAIN_BIN_SUFFIX})
set(CMAKE_ASM_COMPILER ${RISCV_TOOLCHAIN_BIN_PATH}/clang${RISCV_TOOLCHAIN_BIN_SUFFIX})
set(CMAKE_C_COMPILER   ${RISCV_TOOLCHAIN_BIN_PATH}/clang${RISCV_TOOLCHAIN_BIN_SUFFIX})
set(CMAKE_CXX_COMPILER ${RISCV_TOOLCHAIN_BIN_PATH}/clang++${RISCV_TOOLCHAIN_BIN_SUFFIX})
set(CMAKE_LINKER       ${RISCV_TOOLCHAIN_BIN_PATH}/lld${RISCV_TOOLCHAIN_BIN_SUFFIX})

# We must set the OBJCOPY setting into cache so that it's available to the
# whole project. Otherwise, this does not get set into the CACHE and therefore
# the build doesn't know what the OBJCOPY filepath is
set( CMAKE_OBJCOPY      ${RISCV_TOOLCHAIN_BIN_PATH}/llvm-objcopy${RISCV_TOOLCHAIN_BIN_SUFFIX}
     CACHE FILEPATH "The toolchain objcopy command " FORCE )

set( CMAKE_OBJDUMP      ${RISCV_TOOLCHAIN_BIN_PATH}/llvm-objdump${RISCV_TOOLCHAIN_BIN_SUFFIX}
     CACHE FILEPATH "The toolchain objdump command " FORCE )

# Set the common build flags

# Set the CMAKE C flags (which should also be used by the assembler!
set( CMAKE_C_FLAGS "${CMAKE_C_FLAGS} --target=riscv64" )
set( CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -mno-relax -mabi=l64pc128" )
set( CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -march=${CMAKE_SYSTEM_PROCESSOR} -menable-experimental-extensions" )
set( CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -nostartfiles" )
set( CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -nostdlib" )
set( CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -ffreestanding" )

set( CMAKE_C_FLAGS "${CMAKE_C_FLAGS}" CACHE STRING "" )
set( CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS}" CACHE STRING "" )
set( CMAKE_ASM_FLAGS "${CMAKE_C_FLAGS}" CACHE STRING "" )

set( CMAKE_EXE_LINKER_FLAGS   "${CMAKE_EXE_LINKER_FLAGS}  --target=riscv64" )
set( CMAKE_EXE_LINKER_FLAGS   "${CMAKE_EXE_LINKER_FLAGS}  -mno-relax -mabi=l64pc128" )
set( CMAKE_EXE_LINKER_FLAGS   "${CMAKE_EXE_LINKER_FLAGS}  -march=${CMAKE_SYSTEM_PROCESSOR} -menable-experimental-extensions" )
set( CMAKE_EXE_LINKER_FLAGS   "${CMAKE_EXE_LINKER_FLAGS}  -nostartfiles" )
set( CMAKE_EXE_LINKER_FLAGS   "${CMAKE_EXE_LINKER_FLAGS}  -nostdlib" )
set( CMAKE_EXE_LINKER_FLAGS   "${CMAKE_EXE_LINKER_FLAGS}  -ffreestanding" )
set( CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS}" CACHE STRING "" )
