find_package(Python3 3.8 COMPONENTS Interpreter)
add_custom_command(OUTPUT vec_wrappers.h
                   COMMAND ${Python3_EXECUTABLE} ARGS ${CMAKE_CURRENT_SOURCE_DIR}/vector_intrinsic_gen.py ${CMAKE_CURRENT_SOURCE_DIR}/vec_wrappers.h
                   DEPENDS vector_intrinsic_gen.py)
add_custom_command(OUTPUT ${CMAKE_CURRENT_SOURCE_DIR}/test_vector_memcpy.c
                   COMMAND ${Python3_EXECUTABLE} ARGS ${CMAKE_CURRENT_SOURCE_DIR}/vector_gen.py ${CMAKE_CURRENT_SOURCE_DIR}/test_vector_memcpy.c
                   DEPENDS vector_gen.py)


# add_executable(vector_memcpy ${INIT_S} vector_memcpy.c vec_wrappers.h)
add_executable(vector_memcpy ${INIT_S} ${CMAKE_CURRENT_SOURCE_DIR}/test_vector_memcpy.c)
set_target_properties(vector_memcpy PROPERTIES LINK_DEPENDS ${LINK_LD})
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -T ${LINK_LD} -nostdlib")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O1 -fno-inline-functions -ffreestanding -Werror=implicit-function-declaration")
# If you ever try to store vector values on the stack, it will cause problems in the cheri stack bounds processor
# because vectors are variable-size. This is not currently required, but notes have been kept for future reference.
# I tried disabling it with this argument, but that doesn't appear to be exposed to clang.
# If absolutely necessary, recompile clang after changing the default in llvm/lib/CodeGen/CheriBoundAllocas.cpp to Never.
# if("${CMAKE_SYSTEM_PROCESSOR}" MATCHES "xcheri")
#     set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -cheri-stack-bounds=never")
# endif()

# generate an assembly version of the vector_memcpy.c file
# slight hack - CMake represents CMAKE_C_FLAGS as a string, but to pass the individual flags to the compiler we need them as a list.
# separate_arguments splits these into a list C_FLAGS_LIST
separate_arguments(C_FLAGS_LIST NATIVE_COMMAND ${CMAKE_C_FLAGS})
add_custom_command(TARGET vector_memcpy POST_BUILD
                   DEPENDS test_vector_memcpy.c
                   COMMAND ${CMAKE_C_COMPILER} ${C_FLAGS_LIST} -S -o test_vector_memcpy.s ${CMAKE_CURRENT_SOURCE_DIR}/test_vector_memcpy.c)


# extract the binary data from data and text sections to get a binary image of memory
# $(OBJCOPY) -O binary --only-section=.data* --only-section=.text* $< $@
add_custom_command(TARGET vector_memcpy POST_BUILD
                   COMMAND ${CMAKE_OBJCOPY} -O binary --only-section=.data* --only-section=.text* $<TARGET_FILE:vector_memcpy> mem.bin)


# Dump the executable
# $(OBJDUMP) -S -s $< > $@
add_custom_command(TARGET vector_memcpy POST_BUILD
                   COMMAND ${CMAKE_OBJDUMP} -S -s $<TARGET_FILE:vector_memcpy> > $<TARGET_FILE:vector_memcpy>.dump)