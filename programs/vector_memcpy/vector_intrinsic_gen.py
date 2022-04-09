#!/usr/bin/env python3

import argparse
from dataclasses import dataclass
from enum import Enum
from typing import List

class Sew(Enum):
    e8 = 8
    e16 = 16
    e32 = 32
    # e64 = 64

    def get_ecode(x: 'Sew') -> str:
        return f"e{str(x.value)}"

class Lmul(Enum):
    eEighth = 0
    eQuarter = 1
    eHalf = 2
    e1 = 3
    e2 = 4
    e4 = 5
    e8 = 6

    def get_num_regs_times_8(self) -> int:
        return 1 << (self.value)

    def get_code(x: 'Lmul') -> str:
        return {
            Lmul.eEighth: "mf8",
            Lmul.eQuarter: "mf4",
            Lmul.eHalf: "mf2",
            Lmul.e1: "m1",
            Lmul.e2: "m2",
            Lmul.e4: "m4",
            Lmul.e8: "m8",
        }[x]

    def valid_for(self, sew: Sew) -> bool:
        # compute sew / lmul
        # if that's >= 128 then false
        if sew.value / ((2 ** self.value) / 8) >= 128:
            return False
        return True

@dataclass
class VType:
    sew: Sew
    lmul: Lmul

    @staticmethod
    def iterate() -> List['VType']:
        return [
            VType(s, l)
            for l in Lmul
            for s in Sew
            if l.valid_for(s)
        ]

    def get_code(self) -> str:
        return f"{self.sew.get_ecode()}{self.lmul.get_code()}"

    def get_unsigned_type(self) -> str:
        return f"vuint{self.sew.value}{self.lmul.get_code()}_t"

    def get_signed_type(self) -> str:
        return f"vint{self.sew.value}{self.lmul.get_code()}_t"

    def get_vsetvl_func(self) -> str:
        return f"vsetvl_{self.sew.get_ecode()}{self.lmul.get_code()}"

PREFIX="cheri_"

def pick_vector_reg(vtype: VType) -> int:
    # When we do a load/store of LMUL registers, we should be careful which register we use.
    # We don't want to overwrite v0, because that could have a mask
    # If LMUL is integral (i.e. 1,2,4,8), the chosen register needs to be a multiple of LMUL
    # => for LMUL < 1, pick 1, otherwise pick LMUL
    return max(1, vtype.lmul.get_num_regs_times_8() // 8)


def generate_load(vtype: VType, data_type: str, name: str, instr: str):
    return f'''
int {PREFIX}{name}(const void* ptr, size_t vlen) {{
    asm volatile(
        "{instr} v{pick_vector_reg(vtype)}, (ca0)"
        : // we specify output register directly - passing vectors thru the stack doesn't work with CHERI
        : "m"(ptr) // use (ptr) to establish a dependency, but don't use it in the template
    );
    return 0;
}}
'''
def generate_store(vtype: VType, data_type: str, name: str, instr: str):
    return f'''
void {PREFIX}{name}(void* ptr, int fake_data, size_t vlen) {{
    asm volatile(
        "{instr} v{pick_vector_reg(vtype)}, (ca0)"
        : "=m"(ptr) // use (ptr) to establish a dependency, but don't use it in the template
        : // we specify input register directly - passing vectors thru the stack doesn't work with CHERI
        : "memory"
    );
}}
'''

PREAMBLE=f"""#ifndef CHERI_VECTOR_WRAPPERS
#define CHERI_VECTOR_WRAPPERS
#include <stdint.h>
#include <riscv_vector.h>

// This file is autogenerated by vector_intrinsic_gen.py
// It provides equivalents to RISC-V vector load/store instrinsics that are CHERI-compatible.
// Function names are prefixed with '{PREFIX}'

// The reason this is needed at all is because the vector intrinsics haven't been adjusted for CHERI support.
// This can be fixed by using inline assembly, but there's a catch.
// Inline assembly in CHERI-Clang (and also normal Clang trunk) will by default insert offsets in front of memory addresses
// e.g. 'vse8.v v8, 0(ca0)'
// RISC-V Vectors don't support offsets on loads/stores, and reject this.
// To get around this, we wrap each inline asm in a function, 
// and hardcode the address as '(ca0)' with no offset.
// This works because the first argument to each function is the pointer,
// and the RISC-V CHERI ABI puts that argument in the ca0 register every time.

// Define VEC_INTRIN(i) which calls the CHERI version if available
#if __has_feature(capabilities)
#define VEC_INTRIN(i) {PREFIX} ## i
#else
#define VEC_INTRIN(i) i
#endif // __has_feature(capabilities)

// Only generate CHERI versions if we're in CHERI
#if __has_feature(capabilities)
"""

POSTAMBLE=f"""#endif // __has_feature(capabilities)
#endif // CHERI_VECTOR_WRAPPERS"""

def generate_unit_intrinsics() -> str:
    instrinsics = ""
    for vtype in VType.iterate():
        vtype_unit_uload = f"vle{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_iload = f"vle{vtype.sew.value}_v_i{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_ustore = f"vse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_istore = f"vse{vtype.sew.value}_v_i{vtype.sew.value}{vtype.lmul.get_code()}"
        
        vtype_unit_load_instr = f"vle{vtype.sew.value}.v"
        vtype_unit_store_instr = f"vse{vtype.sew.value}.v"

        instrinsics += generate_load(vtype, vtype.get_unsigned_type(), vtype_unit_uload, vtype_unit_load_instr)
        instrinsics += generate_store(vtype, vtype.get_unsigned_type(), vtype_unit_ustore, vtype_unit_store_instr)
        instrinsics += generate_load(vtype, vtype.get_signed_type(), vtype_unit_iload, vtype_unit_load_instr)
        instrinsics += generate_store(vtype, vtype.get_signed_type(), vtype_unit_istore, vtype_unit_store_instr)

    return instrinsics

def generate_intrinsics() -> str:
    return PREAMBLE + generate_unit_intrinsics() + POSTAMBLE

if __name__ == '__main__':
    parser = argparse.ArgumentParser("vector_intrinsic_gen", description="Generator for CHERI-compatible vector intrinsics")
    parser.add_argument("output_cpp", type=str)

    args = parser.parse_args()

    tests = generate_intrinsics()
    with open(args.output_cpp, "w") as f:
        f.write(tests)
