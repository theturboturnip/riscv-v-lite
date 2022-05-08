add_executable(vector_memcpy_pointers ${INIT_S} vector_memcpy_pointers.cpp)
set_target_properties(vector_memcpy_pointers PROPERTIES LINK_DEPENDS ${LINK_LD})
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -T ${LINK_LD} -nostdlib")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O1 -g -fno-inline-functions -ffreestanding -Werror=implicit-function-declaration")

# generate an assembly version of the vector_memcpy_pointers.c file
# slight hack - CMake represents CMAKE_C_FLAGS as a string, but to pass the individual flags to the compiler we need them as a list.
# separate_arguments splits these into a list C_FLAGS_LIST
separate_arguments(C_FLAGS_LIST NATIVE_COMMAND ${CMAKE_CXX_FLAGS})
add_custom_command(TARGET vector_memcpy_pointers POST_BUILD
                   DEPENDS vector_memcpy_pointers.cpp
                   COMMAND ${CMAKE_CXX_COMPILER} ${C_FLAGS_LIST} -S -o vector_memcpy_pointers.s ${CMAKE_CURRENT_SOURCE_DIR}/vector_memcpy_pointers.cpp)


# extract the binary data from data and text sections to get a binary image of memory
# $(OBJCOPY) -O binary --only-section=.data* --only-section=.text* $< $@
add_custom_command(TARGET vector_memcpy_pointers POST_BUILD
                   COMMAND ${CMAKE_OBJCOPY} -O binary --only-section=.data* --only-section=.text* $<TARGET_FILE:vector_memcpy_pointers> mem.bin)


# Dump the executable
# $(OBJDUMP) -S -s $< > $@
add_custom_command(TARGET vector_memcpy_pointers POST_BUILD
                   COMMAND ${CMAKE_OBJDUMP} -S -s $<TARGET_FILE:vector_memcpy_pointers> > $<TARGET_FILE:vector_memcpy_pointers>.dump)