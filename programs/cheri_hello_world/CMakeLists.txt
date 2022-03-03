add_executable(cheri_hello_world init.s hello_world.c)
set_target_properties(cheri_hello_world PROPERTIES LINK_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/link.ld)
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -T ${CMAKE_CURRENT_SOURCE_DIR}/link.ld -nostdlib")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O1 -fno-inline-functions -ffreestanding -Werror=implicit-function-declaration")

# generate an assembly version of the hello_world.c file
# slight hack - CMake represents CMAKE_C_FLAGS as a string, but to pass the individual flags to the compiler we need them as a list.
# separate_arguments splits these into a list C_FLAGS_LIST
separate_arguments(C_FLAGS_LIST NATIVE_COMMAND ${CMAKE_C_FLAGS})
add_custom_command(TARGET cheri_hello_world POST_BUILD
                   DEPENDS hello_world.c
                   COMMAND ${CMAKE_C_COMPILER} ${C_FLAGS_LIST} -S -o hello_world.s ${CMAKE_CURRENT_SOURCE_DIR}/hello_world.c)


# extract the binary data from data and text sections to get a binary image of memory
# $(OBJCOPY) -O binary --only-section=.data* --only-section=.text* $< $@
add_custom_command(TARGET cheri_hello_world POST_BUILD
                   COMMAND ${CMAKE_OBJCOPY} -O binary --only-section=.data* --only-section=.text* $<TARGET_FILE:cheri_hello_world> mem.bin)
