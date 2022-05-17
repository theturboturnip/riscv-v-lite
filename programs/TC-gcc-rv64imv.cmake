# usage
# cmake -DCMAKE_TOOLCHAIN_FILE=../TC-gcc-rv32iv.cmake ../

# The Generic system name is used for embedded targets (targets without OS) in
# CMake
set( CMAKE_SYSTEM_NAME          Generic )
set( CMAKE_SYSTEM_PROCESSOR     rv64imv   )
set( CMAKE_EXECUTABLE_SUFFIX    ".elf" )
set( CMAKE_EXECUTABLE_SUFFIX_C    ".elf" )
set( CMAKE_EXECUTABLE_SUFFIX_CXX    ".elf" )

# Look for RISC-V github GCC
# https://github.com/riscv/riscv-gnu-toolchain
FIND_FILE( RISCV_GCC_COMPILER_EXE_MAYBE "riscv64-unknown-elf-gcc.exe" PATHS ENV INCLUDE)
FIND_FILE( RISCV_GCC_COMPILER_MAYBE     "riscv64-unknown-elf-gcc"     PATHS ENV INCLUDE)

# Select which is found
if (EXISTS ${RISCV_GCC_COMPILER_MAYBE})
set( RISCV_GCC_COMPILER ${RISCV_GCC_COMPILER_MAYBE})
elseif (EXISTS ${RISCV_GCC_COMPILER_EXE_MAYBE})
set( RISCV_GCC_COMPILER ${RISCV_GCC_COMPILER_EXE_MAYBE})
else()
message(FATAL_ERROR "RISC-V GCC not found. ${RISCV_GCC_COMPILER_MAYBE} ${RISCV_GCC_COMPILER_EXE_MAYBE}")
endif()

message( "RISC-V GCC found: ${RISCV_GCC_COMPILER}")

get_filename_component(RISCV_TOOLCHAIN_BIN_PATH ${RISCV_GCC_COMPILER} DIRECTORY)
get_filename_component(RISCV_TOOLCHAIN_BIN_GCC ${RISCV_GCC_COMPILER} NAME_WE)
get_filename_component(RISCV_TOOLCHAIN_BIN_EXT ${RISCV_GCC_COMPILER} EXT)

message( "RISC-V GCC Path: ${RISCV_TOOLCHAIN_BIN_PATH}" )

# Get the "cross compile" string from the detected gcc path
# Replace "-gcc" with "-"
# e.g. "riscv32-unknown-elf-gcc" -> "riscv32-unknown-elf-", a common prefix for other tools e.g. "riscv32-unknown-elf-ar"
STRING(REGEX REPLACE "\-gcc" "-" CROSS_COMPILE ${RISCV_TOOLCHAIN_BIN_GCC})
message( "RISC-V Cross Compile: ${CROSS_COMPILE}" )

# where is the target environment located
set(CMAKE_FIND_ROOT_PATH ${RISCV_TOOLCHAIN_BIN_PATH}../)

# adjust the default behavior of the FIND_XXX() commands:
# search programs in the host environment
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
# search headers and libraries in the target environment
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)

# specify the cross compiler. We force the compiler so that CMake doesn't
# attempt to build a simple test program as this will fail without us using
# the -nostartfiles option on the command line
set(CMAKE_ASM_COMPILER ${CROSS_COMPILE}gcc)
set(CMAKE_AR           ${CROSS_COMPILE}ar )
set(CMAKE_ASM_COMPILER ${CROSS_COMPILE}gcc)
set(CMAKE_C_COMPILER   ${CROSS_COMPILE}gcc)
set(CMAKE_CXX_COMPILER ${CROSS_COMPILE}g++)
set(CMAKE_LINKER ${CROSS_COMPILE}ld)

# We must set the OBJCOPY setting into cache so that it's available to the
# whole project. Otherwise, this does not get set into the CACHE and therefore
# the build doesn't know what the OBJCOPY filepath is
set( CMAKE_OBJCOPY      ${RISCV_TOOLCHAIN_BIN_PATH}/${CROSS_COMPILE}objcopy
     CACHE FILEPATH "The toolchain objcopy command " FORCE )

set( CMAKE_OBJDUMP      ${RISCV_TOOLCHAIN_BIN_PATH}/${CROSS_COMPILE}objdump
     CACHE FILEPATH "The toolchain objdump command " FORCE )

# Set the common build flags

# Set the CMAKE C flags (which should also be used by the assembler!
set( CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -march=${CMAKE_SYSTEM_PROCESSOR} -mabi=lp64" )

set( CMAKE_C_FLAGS "${CMAKE_C_FLAGS}" CACHE STRING "" )
set( CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS}" CACHE STRING "" )
set( CMAKE_ASM_FLAGS "${CMAKE_C_FLAGS}" CACHE STRING "" )
set( CMAKE_EXE_LINKER_FLAGS   "${CMAKE_EXE_LINKER_FLAGS}  -nostartfiles   " CACHE STRING "")
