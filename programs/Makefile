# This script builds

# build directory
BUILD = build
# the programs we're going to build
PROGRAMS := vector_memcpy cheri_hello_world

# The source files for all programs
# If any of these are changed, CMake should be invoked for each toolchain
ALL_PROGRAMS_SOURCES := $(wildcard *.c)
ALL_PROGRAMS_CMAKELISTS := $(wildcard */CMakeLists.txt)

# The CMake toolchains we will use
TOOLCHAIN_FILES := $(wildcard TC-*.cmake)
# Get the names from the toolchains
TOOLCHAINS := $(patsubst TC-%.cmake,%,$(TOOLCHAIN_FILES))
# # Build folders for each toolchain/program combination
# TOOLCHAIN_PROGRAM_BUILD_FOLDERS := $(foreach TC,$(TOOLCHAINS),$(foreach PROGRAM,$(PROGRAMS),$(BUILD)/$(TC)/$(PROGRAM)))
# The "output" files generated for each toolchain
TOOLCHAIN_OUTPUTS := $(foreach TC,$(TOOLCHAINS),$(BUILD)/$(TC)/CMakeCache.txt)
# The build directories for each toolchain
TOOLCHAIN_DIRS := $(foreach TC,$(TOOLCHAINS),$(BUILD)/$(TC))

all: dirs $(TOOLCHAIN_OUTPUTS)

# Each CMake invocation for a toolchain creates a CMakeCache.txt
$(BUILD)/%/CMakeCache.txt: TC-%.cmake $(ALL_PROGRAMS_SOURCES) $(ALL_PROGRAMS_CMAKELISTS)
	# $* = pattern "stem", the bit matched with % in the rule
	# Tell CMake to build the programs with a specific toolchain
	# If it fails, don't worry - maybe the user doesn't have this specific toolchain installed, and should still try the others
	- (cd $(BUILD)/$* && cmake -DCMAKE_TOOLCHAIN_FILE=../../TC-$*.cmake ../../ && cmake --build .)


# $(BUILD)/mem.bin $(BUILD)/$(PROGRAM).dump $(BUILD)/$(PROGRAM).s



# # build an elf executable
# $(BUILD)/$(PROGRAM).dump: $(BUILD)/$(PROGRAM).elf
# 	$(OBJDUMP) -S -s $< > $@

# # extract the binary data from data and text sections to get a binary image of memory
# $(BUILD)/mem.bin: $(BUILD)/$(PROGRAM).elf
# 	$(OBJCOPY) -O binary --only-section=.data* --only-section=.text* $< $@

# make software project folder
dirs:
	mkdir -p $(BUILD) $(TOOLCHAIN_DIRS)

# 'clean' rule: delete all the files so we can start afresh
clean:
	rm -rf $(TOOLCHAIN_DIRS)

rebuild: clean all