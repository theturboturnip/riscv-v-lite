add_executable(hello_world ${INIT_S} hello_world.c)
set_target_properties(hello_world PROPERTIES LINK_DEPENDS ${LINK_LD})
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -T ${LINK_LD} -nostdlib")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O1 -fno-inline-functions -ffreestanding -Werror=implicit-function-declaration")

# generate an assembly version of the hello_world.c file
# slight hack - CMake represents CMAKE_C_FLAGS as a string, but to pass the individual flags to the compiler we need them as a list.
# separate_arguments splits these into a list C_FLAGS_LIST
separate_arguments(C_FLAGS_LIST NATIVE_COMMAND ${CMAKE_C_FLAGS})
add_custom_command(TARGET hello_world POST_BUILD
                   DEPENDS hello_world.c
                   COMMAND ${CMAKE_C_COMPILER} ${C_FLAGS_LIST} -S -o hello_world.s ${CMAKE_CURRENT_SOURCE_DIR}/hello_world.c)


# extract the binary data from data and text sections to get a binary image of memory
# $(OBJCOPY) -O binary --only-section=.data* --only-section=.text* $< $@
add_custom_command(TARGET hello_world POST_BUILD
                   COMMAND ${CMAKE_OBJCOPY} -O binary --only-section=.data* --only-section=.text* --only-section=.srodata* $<TARGET_FILE:hello_world> mem.bin)


# Dump the executable
# $(OBJDUMP) -S -s $< > $@
add_custom_command(TARGET hello_world POST_BUILD
                   COMMAND ${CMAKE_OBJDUMP} -S -s $<TARGET_FILE:hello_world> > $<TARGET_FILE:hello_world>.dump)