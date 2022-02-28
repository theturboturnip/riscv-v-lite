add_executable(vector_memcpy init.s vector_memcpy.c)
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -T ${CMAKE_CURRENT_SOURCE_DIR}/link.ld -nostdlib")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O1 -fno-inline-functions -ffreestanding -Werror=implicit-function-declaration")

# generate an assembly version of the vector_memcpy.c file
# slight hack - CMake represents CMAKE_C_FLAGS as a string, but to pass the individual flags to the compiler we need them as a list.
# separate_arguments splits these into a list C_FLAGS_LIST
separate_arguments(C_FLAGS_LIST NATIVE_COMMAND ${CMAKE_C_FLAGS})
add_custom_command(TARGET vector_memcpy POST_BUILD
                   DEPENDS vector_memcpy.c
                   COMMAND ${CMAKE_C_COMPILER} ${C_FLAGS_LIST} -S -o vector_memcpy.s ${CMAKE_CURRENT_SOURCE_DIR}/vector_memcpy.c)


# extract the binary data from data and text sections to get a binary image of memory
# $(OBJCOPY) -O binary --only-section=.data* --only-section=.text* $< $@
add_custom_command(TARGET vector_memcpy POST_BUILD
                   COMMAND ${CMAKE_OBJCOPY} -O binary --only-section=.data* --only-section=.text* $<TARGET_FILE:vector_memcpy> mem.bin)


# Dump the executable
# $(OBJDUMP) -S -s $< > $@
add_custom_command(TARGET vector_memcpy POST_BUILD
                   COMMAND ${CMAKE_OBJDUMP} -S -s $<TARGET_FILE:vector_memcpy> > $<TARGET_FILE:vector_memcpy>.dump)