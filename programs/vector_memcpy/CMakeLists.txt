find_package(Python3 3.8 COMPONENTS Interpreter)
add_custom_command(OUTPUT vec_wrappers.hpp
                   COMMAND ${Python3_EXECUTABLE} ARGS ${CMAKE_CURRENT_SOURCE_DIR}/vector_intrinsic_gen.py ${CMAKE_CURRENT_SOURCE_DIR}/vec_wrappers.hpp)

add_executable(vector_memcpy ${INIT_S} vector_memcpy.cpp vec_wrappers.hpp)
set_target_properties(vector_memcpy PROPERTIES LINK_DEPENDS ${LINK_LD})
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -T ${LINK_LD} -nostdlib")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O1 -fno-inline-functions -ffreestanding -Werror=implicit-function-declaration")

# generate an assembly version of the vector_memcpy.c file
# slight hack - CMake represents CMAKE_C_FLAGS as a string, but to pass the individual flags to the compiler we need them as a list.
# separate_arguments splits these into a list C_FLAGS_LIST
separate_arguments(C_FLAGS_LIST NATIVE_COMMAND ${CMAKE_CXX_FLAGS})
add_custom_command(TARGET vector_memcpy POST_BUILD
                   DEPENDS vector_memcpy.cpp
                   COMMAND ${CMAKE_CXX_COMPILER} ${C_FLAGS_LIST} -S -o vector_memcpy.s ${CMAKE_CURRENT_SOURCE_DIR}/vector_memcpy.cpp)


# extract the binary data from data and text sections to get a binary image of memory
# $(OBJCOPY) -O binary --only-section=.data* --only-section=.text* $< $@
add_custom_command(TARGET vector_memcpy POST_BUILD
                   COMMAND ${CMAKE_OBJCOPY} -O binary --only-section=.data* --only-section=.text* $<TARGET_FILE:vector_memcpy> mem.bin)


# Dump the executable
# $(OBJDUMP) -S -s $< > $@
add_custom_command(TARGET vector_memcpy POST_BUILD
                   COMMAND ${CMAKE_OBJDUMP} -S -s $<TARGET_FILE:vector_memcpy> > $<TARGET_FILE:vector_memcpy>.dump)