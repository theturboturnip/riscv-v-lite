#!/usr/bin/env python3

import argparse
from dataclasses import dataclass
from enum import Enum
from typing import List
import math

class Sew(Enum):
    e8 = 8
    e16 = 16
    e32 = 32
    e64 = 64

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

    def is_frac(self) -> bool:
        return self.value < Lmul.e1.value

    def get_num_regs_times_8(self) -> int:
        return 1 << (self.value)

    # get_num_regs_times_8 contains the fractional - this counts the number of registers consumed by a single Lmul group
    def get_num_regs_consumed(self) -> int:
        if self.value <= Lmul.e1.value:
            return 1
        elif self == Lmul.e2:
            return 2
        elif self == Lmul.e4:
            return 4
        elif self == Lmul.e8:
            return 8

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

    def get_unsigned_elem_type(self) -> str:
        return f"uint{self.sew.value}_t"

    def get_unsigned_type(self) -> str:
        return f"vuint{self.sew.value}{self.lmul.get_code()}_t"

    def get_signed_type(self) -> str:
        return f"vint{self.sew.value}{self.lmul.get_code()}_t"

    def get_vsetvl_func(self) -> str:
        return f"vsetvl_{self.sew.get_ecode()}{self.lmul.get_code()}"

    def get_sew_lmul_ratio(self) -> int:
        return math.trunc(self.sew.value * 8.0 / self.lmul.get_num_regs_times_8())

    def get_mask_type(self) -> str:
        return f"vbool{self.get_sew_lmul_ratio()}_t"

PREFIX="cheri_"

def pick_vector_reg(vtype: VType) -> int:
    # When we do a load/store of LMUL registers, we should be careful which register we use.
    # We don't want to overwrite v0, because that could have a mask
    # If LMUL is integral (i.e. 1,2,4,8), the chosen register needs to be a multiple of LMUL
    # => for LMUL < 1, pick 1, otherwise pick LMUL
    return max(1, vtype.lmul.get_num_regs_times_8() // 8)


def generate_unit_load(vtype: VType, data_type: str, name: str, instr: str):
    return f'''
int {PREFIX}{name}(const void* ptr, size_t vlen) {{
    asm volatile(
        "{instr} v{pick_vector_reg(vtype)}, (%0)"
        : // we specify output register directly - passing vectors thru the stack doesn't work with CHERI
        : "C"(ptr) // use (ptr) to establish a dependency, but don't use it in the template
    );
    return 0;
}}
'''
def generate_unit_store(vtype: VType, data_type: str, name: str, instr: str):
    return f'''
void {PREFIX}{name}(void* ptr, int fake_data, size_t vlen) {{
    asm volatile(
        "{instr} v{pick_vector_reg(vtype)}, (%0)"
        : // no "outputs" (we write out to *ptr but can't do "=m"(*ptr))
        : "C"(ptr) // input vector register specified directly - passing vectors thru the stack doesn't work with CHERI
        : "memory"
    );
}}
'''

def generate_strided_load(vtype: VType, data_type: str, name: str, instr: str):
    return f'''
int {PREFIX}{name}(const void* ptr, ptrdiff_t stride, size_t vlen) {{
    asm volatile(
        "{instr} v{pick_vector_reg(vtype)}, (%0), %1"
        : // we specify output register directly - passing vectors thru the stack doesn't work with CHERI
        : "C"(ptr), "r"(stride)
    );
    return 0;
}}
'''
def generate_strided_store(vtype: VType, data_type: str, name: str, instr: str):
    return f'''
void {PREFIX}{name}(void* ptr, ptrdiff_t stride, int fake_data, size_t vlen) {{
    asm volatile(
        "{instr} v{pick_vector_reg(vtype)}, (%0), %1"
        : // no "outputs" (we write out to *ptr but can't do "=m"(*ptr))
        : "C"(ptr), "r"(stride) // input vector register specified directly - passing vectors thru the stack doesn't work with CHERI
        : "memory"
    );
}}
'''

def generate_fof_load(vtype: VType, data_type: str, name: str, instr: str):
    return f'''
int {PREFIX}{name}(const void* ptr, size_t* new_vlen, size_t vlen) {{
    // do the fof load and then read VL into *new_vlen
    size_t new_vlen_val;
    asm volatile(
        "{instr} v{pick_vector_reg(vtype)}, (%1)\\n\\tcsrr %0, vl"
        : "=r"(new_vlen_val)
        : "C"(ptr)
    );
    *new_vlen = new_vlen_val;
    return 0;
}}
'''

PREAMBLE=f"""#ifndef CHERI_VECTOR_WRAPPERS
#define CHERI_VECTOR_WRAPPERS
#include <stdint.h>
#include <riscv_vector.h>

// This file is autogenerated by vector_intrinsic_gen.py
// It provides equivalents to RISC-V vector load/store intrinsics that are CHERI-compatible.
// Function names are prefixed with '{PREFIX}'
// The reason this is needed at all is because the vector intrinsics haven't been adjusted for CHERI support.

// Define VEC_INTRIN(i) which calls the CHERI version if available
#if __has_feature(capabilities)
#define VEC_INTRIN(i) {PREFIX} ## i
#define VEC_TYPE(T) int
#else
#define VEC_INTRIN(i) i
#define VEC_TYPE(T) T
#endif // __has_feature(capabilities)

// Only generate CHERI versions if we're in CHERI
#if __has_feature(capabilities)
"""

POSTAMBLE=f"""#endif // __has_feature(capabilities)
#endif // CHERI_VECTOR_WRAPPERS"""

def generate_unit_intrinsics() -> str:
    intrinsics = ""
    for vtype in VType.iterate():
        vtype_unit_uload = f"vle{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_iload = f"vle{vtype.sew.value}_v_i{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_ustore = f"vse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_istore = f"vse{vtype.sew.value}_v_i{vtype.sew.value}{vtype.lmul.get_code()}"
        
        vtype_unit_load_instr = f"vle{vtype.sew.value}.v"
        vtype_unit_store_instr = f"vse{vtype.sew.value}.v"

        intrinsics += generate_unit_load(vtype, vtype.get_unsigned_type(), vtype_unit_uload, vtype_unit_load_instr)
        intrinsics += generate_unit_store(vtype, vtype.get_unsigned_type(), vtype_unit_ustore, vtype_unit_store_instr)
        intrinsics += generate_unit_load(vtype, vtype.get_signed_type(), vtype_unit_iload, vtype_unit_load_instr)
        intrinsics += generate_unit_store(vtype, vtype.get_signed_type(), vtype_unit_istore, vtype_unit_store_instr)

    return intrinsics

def generate_strided_intrinsics() -> str:
    intrinsics = ""
    for vtype in VType.iterate():
        vtype_strided_uload = f"vlse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_strided_iload = f"vlse{vtype.sew.value}_v_i{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_strided_ustore = f"vsse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_strided_istore = f"vsse{vtype.sew.value}_v_i{vtype.sew.value}{vtype.lmul.get_code()}"
        
        vtype_strided_load_instr = f"vlse{vtype.sew.value}.v"
        vtype_strided_store_instr = f"vsse{vtype.sew.value}.v"

        intrinsics += generate_strided_load(vtype, vtype.get_unsigned_type(), vtype_strided_uload, vtype_strided_load_instr)
        intrinsics += generate_strided_store(vtype, vtype.get_unsigned_type(), vtype_strided_ustore, vtype_strided_store_instr)
        intrinsics += generate_strided_load(vtype, vtype.get_signed_type(), vtype_strided_iload, vtype_strided_load_instr)
        intrinsics += generate_strided_store(vtype, vtype.get_signed_type(), vtype_strided_istore, vtype_strided_store_instr)

    return intrinsics

def generate_fof_intrinsics() -> str:
    intrinsics = ""
    for vtype in VType.iterate():
        vtype_fof_uload = f"vle{vtype.sew.value}ff_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_fof_iload = f"vle{vtype.sew.value}ff_v_i{vtype.sew.value}{vtype.lmul.get_code()}"
        
        vtype_fof_load_instr = f"vle{vtype.sew.value}ff.v"

        intrinsics += generate_fof_load(vtype, vtype.get_unsigned_type(), vtype_fof_uload, vtype_fof_load_instr)
        intrinsics += generate_fof_load(vtype, vtype.get_signed_type(), vtype_fof_iload, vtype_fof_load_instr)

    return intrinsics

def generate_intrinsics() -> str:
    return (PREAMBLE + 
        generate_unit_intrinsics() +
        generate_strided_intrinsics() +
        generate_fof_intrinsics() +
        POSTAMBLE
    )

if __name__ == '__main__':
    parser = argparse.ArgumentParser("vector_intrinsic_gen", description="Generator for CHERI-compatible vector intrinsics")
    parser.add_argument("output_cpp", type=str)

    args = parser.parse_args()

    tests = generate_intrinsics()
    with open(args.output_cpp, "w") as f:
        f.write(tests)
