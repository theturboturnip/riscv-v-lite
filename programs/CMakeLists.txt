cmake_minimum_required (VERSION 3.9)
project (RISCV_V_LITE_PROGRAMS)

enable_language(ASM)

if("${CMAKE_SYSTEM_PROCESSOR}" MATCHES "xcheri")
    add_subdirectory(cheri_hello_world)
else()
    add_subdirectory(vector_memcpy)
endif()