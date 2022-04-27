# This script builds

# build directory
BUILD = build
# the programs we're going to build
PROGRAMS := vector_memcpy hello_world

# The associated files for all programs
ALL_PROGRAMS_SOURCES := $(foreach P,$(PROGRAMS),$(wildcard $(P)/*))
ALL_PROGRAMS_CMAKELISTS := $(wildcard */CMakeLists.txt) CMakeLists.txt

# The CMake toolchains we will use
TOOLCHAIN_FILES := $(wildcard TC-*.cmake)
# Get the names from the toolchains
TOOLCHAINS := $(patsubst TC-%.cmake,%,$(TOOLCHAIN_FILES))
# The build directories for each toolchain
TOOLCHAIN_DIRS := $(foreach TC,$(TOOLCHAINS),$(BUILD)/$(TC))
# "cmakestamp" = a file that is touch-d after CMake has generated the build system for a given toolchain
# CMake should only be re-invoked if this "cmakestamp" is out of date vs. the CMakeLists and toolchain files.
TOOLCHAIN_CMAKESTAMPS := $(foreach TC,$(TOOLCHAINS),$(BUILD)/$(TC)/.cmakestamp)
# "execstamp" = a file that is touch-d after executables have been compiled under a given toolchain
# A toolchain should only re-compile its executables if the executable sources change,
# or the build system for that toolchain changes (i.e. the .cmakestamp changes)
TOOLCHAIN_EXECSTAMPS := $(foreach TC,$(TOOLCHAINS),$(BUILD)/$(TC)/.execstamp)

all: dirs $(TOOLCHAIN_EXECSTAMPS)

# Rebuild the build systems for each program for a toolchain
# if that toolchain changes OR any CMakeLists.txt changes
$(BUILD)/%/.cmakestamp: TC-%.cmake $(ALL_PROGRAMS_CMAKELISTS)
	@# $* = pattern "stem", the bit matched with % in the rule
	@# Tell CMake to create the build system for a specific toolchain.
	@# If it fails, don't worry - maybe the user doesn't have this specific toolchain installed, and should still try the others
	@# cmakestamp will only be generated if the toolchain succeeds.
	@echo "Running cmake for $*"
	- (cd $(BUILD)/$* && cmake -DCMAKE_TOOLCHAIN_FILE=../../TC-$*.cmake ../../ && touch .cmakestamp)

# Prevent Make from removing cmakestamps automatically
.SECONDARY: $(TOOLCHAIN_CMAKESTAMPS)

# Recompile executables if the buildsystem has changed (e.g. if any toolchains or CMakeLists change),
# or if any program sources change
$(BUILD)/%/.execstamp: $(BUILD)/%/.cmakestamp $(ALL_PROGRAMS_SOURCES) TC-%.cmake $(ALL_PROGRAMS_CMAKELISTS)
	@# Go to the build directory, check that .cmakestamp exists, then invoke the build system
	@# (if the toolchain failed, .cmakestamp won't have been generated.)
	@echo "Building for $*"
	@cd $(BUILD)/$* && [ -f .cmakestamp ] \
		&& (cmake --build . -- --no-print-directory && touch .execstamp) \
		|| echo ".cmakestamp not found for $*, toolchain must have failed. not building..."

# make software project folder
dirs:
	mkdir -p $(BUILD) $(TOOLCHAIN_DIRS)

# 'clean' rule: delete all the files so we can start afresh
clean:
	rm -rf $(TOOLCHAIN_DIRS)

rebuild: clean all