cmake_minimum_required (VERSION 3.9)
project (RISCV_V_LITE_PROGRAMS)

enable_language(ASM)

if("${CMAKE_SYSTEM_PROCESSOR}" MATCHES "xcheri")
    set(INIT_S ${CMAKE_CURRENT_LIST_DIR}/cheri_init.s)
    set(LINK_LD ${CMAKE_CURRENT_LIST_DIR}/cheri_link.ld)
else()
    set(INIT_S ${CMAKE_CURRENT_LIST_DIR}/init.s)
    set(LINK_LD ${CMAKE_CURRENT_LIST_DIR}/link.ld)
endif()

add_subdirectory(hello_world)
add_subdirectory(vector_memcpy)
