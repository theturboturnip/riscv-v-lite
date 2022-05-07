cmake_minimum_required (VERSION 3.9)
project (RISCV_V_LITE_PROGRAMS)

enable_language(ASM)

if("${USE_CHERI_LINK_SCRIPT}" MATCHES "TRUE")
    set(INIT_S ${CMAKE_CURRENT_LIST_DIR}/cheri_init.s)
    set(LINK_LD ${CMAKE_CURRENT_LIST_DIR}/cheri_link.ld)
else()
    set(INIT_S ${CMAKE_CURRENT_LIST_DIR}/init.s)
    set(LINK_LD ${CMAKE_CURRENT_LIST_DIR}/link.ld)
endif()

add_subdirectory(hello_world)
add_subdirectory(vector_memcpy)
if ("${CMAKE_SYSTEM_PROCESSOR}" MATCHES "xcheri")
else()
    # Only compile vector_memcpy_old on not-CHERI settings
    add_subdirectory(vector_memcpy_old)
endif()
