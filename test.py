#!/usr/bin/env python3.8

from dataclasses import dataclass
from typing import Dict, List
import json
import subprocess
import re
import pty
import os, sys

all_tests = [
    "hello_world",
    "vector_memcpy",
    "vector_memcpy_old",
    "vector_memcpy_pointers",
]
cheri_tests = [
    "hello_world",
    "vector_memcpy",
    # vector_memcpy_old removed - not adapted to CHERI
    "vector_memcpy_pointers",
]
available_tests_per_arch = {
    ("gcc", "rv64imv"): all_tests,
    ("llvm-13", "rv32imv"): all_tests,
    ("llvm-13", "rv64imv"): all_tests,
    ("llvm-trunk", "rv64imv"): all_tests,
    ("llvm-13", "rv64imvxcheri"): cheri_tests,
    ("llvm-13", "rv64imvxcheri-int"): cheri_tests,
}

def load_test_metadata_json(test: str) -> Dict[int, str]:
    with open(f"./programs/{test}/test_list.json") as f:
        raw_json = json.loads(f.read())

    return {
        int(k): v["test"]
        for k,v in raw_json.items()
    }

@dataclass
class TestProgramResult:
    compiler: str
    arch: str
    test_program: str

    output: List[str]

    crashed: bool
    successful: bool
    unsuccessful_indices: List[int]

def run_test(compiler: str, arch: str, test_program: str, use_elf: bool):
    if use_elf:
        memory_file = f"../programs/build/{compiler}-{arch}/{test_program}/{test_program}.elf"
    else:
        memory_file = f"../programs/build/{compiler}-{arch}/{test_program}/mem.bin"
    
    process = subprocess.run([
        "cargo", "run", "direct", arch, memory_file
    ], cwd="./rsim/", stdout=subprocess.PIPE, stderr=subprocess.STDOUT)

    output = process.stdout.decode('utf-8').split("\n")

    if any([line.startswith("All tests ran were successful") for line in output]):
        crashed = False
        successful = True
        unsuccessful_indices = []
    elif any([line.startswith("Not all tests were successful.") for line in output]):
        crashed = False
        successful = False

        try:
            unsuccessful_list_start = output.index("Unsuccessful Indices:")
        except IndexError:
            raise ValueError("Couldn't find list of unsuccessful tests in an unsuccessful test run")

        unsuccessful_indices = [
            int(line)
            for line in output[unsuccessful_list_start+1:]
            if line.strip() and not line.strip().startswith("Finished")
        ]
    else:
        crashed = True
        successful = False
        unsuccessful_indices = []

    return TestProgramResult(
        compiler=compiler,
        arch=arch,
        test_program=test_program,

        output=output,

        crashed=crashed,
        successful=successful,
        unsuccessful_indices=unsuccessful_indices,
    )

def run_tests(output_path):
    test_metadata = {
        test: load_test_metadata_json(test)
        for test in all_tests
    }

    test_results = []

    for ((compiler, arch), tests) in available_tests_per_arch.items():
        for test in tests:
            use_elf = arch.find("xcheri") != -1
            result = run_test(compiler, arch, test, use_elf)
            test_results.append(result)

    with open(output_path, "w") as f: 
        for result in test_results:
            if result.crashed:
                print(f"{result.compiler}-{result.arch} [{result.test_program}]: CRASHED")
                print("\n".join(result.output))

                for index, test in test_metadata[result.test_program].items():
                    ran = "Crashed"
                    successful = "-"
                    
                    print(f"{result.compiler}\t{result.arch}\t{result.test_program}\t{test}\t{ran}\t{successful}", file=f)
            elif result.successful:
                print(f"{result.compiler}-{result.arch} [{result.test_program}]: SUCCESS")
                number_match = next(re.search(r"All tests ran were successful: 0x([0-9a-fA-F]+)", s) for s in result.output if s.startswith("All tests ran were successful")) 
                test_outcome = int(number_match.group(1), base=16)
                for index, test in test_metadata[result.test_program].items():
                    ran = ((test_outcome >> index) & 1) == 1
                    successful = True if ran else "-"
                    
                    print(f"{result.compiler}\t{result.arch}\t{result.test_program}\t{test}\t{ran}\t{successful}", file=f)
            else:
                print(f"{result.compiler}-{result.arch} [{result.test_program}]: UNSUCCESSFUL")
                print("\n".join(result.output))
                print("Failed:")
                for index in result.unsuccessful_indices:
                    print(test_metadata[result.test_program][index])
                    
                number_match = next(re.search(r"Ran\s+0x([0-9a-fA-F]+)", s) for s in result.output if s.startswith("Ran")) 
                test_outcome = int(number_match.group(1), base=16)
                for index, test in test_metadata[result.test_program].items():
                    ran = ((test_outcome >> index) & 1) == 1
                    successful = index not in result.unsuccessful_indices
                    
                    print(f"{result.compiler}\t{result.arch}\t{result.test_program}\t{test}\t{ran}\t{successful}", file=f)

def main():
    import time
    timestr = time.strftime("%Y%m%d-%H%M%S")
    run_tests(f"./test_results/{timestr}.tsv")

if __name__ == '__main__':
    main()