#!/usr/bin/env python3.8

import argparse
import contextlib
from dataclasses import dataclass
from enum import Enum
from io import StringIO
from typing import Any, ContextManager, Dict, List, Optional, Tuple, Union

from vector_intrinsic_gen import Sew, Lmul, VType

class CppBuilder:
    """
    Helper class to generate C++ code while keeping proper formatting.
    From https://schemingdeveloper.com/2019/03/31/generating-c-code-using-python-and-cmake/
    """

    def __init__(self, indent_len=4):
        self._buffer = [StringIO()]
        self._indentation_level = 0
        self._indent_spaces = ' ' * indent_len
        self._indent_next = False

    def get_value(self) -> str:
        assert len(self._buffer) == 1
        return self._buffer[-1].getvalue()

    def save(self, file_out: str) -> None:
        assert len(self._buffer) == 1
        with open(file_out, 'w') as f:
            f.write(self._buffer[-1].getvalue())

    @contextlib.contextmanager
    def indent(self) -> ContextManager:
        self.push_indent()
        try:
            yield
        finally:
            self.pop_indent()

    def push_indent(self) -> None:
        self._indentation_level += 1

    def pop_indent(self) -> None:
        self._indentation_level = max(self._indentation_level - 1, 0)

    @property
    def indentation(self):
        return self._indent_spaces * self._indentation_level

    def write(self, code: str) -> None:
        if not code:
            return

        if self._indent_next:
            self._buffer[-1].write(self.indentation)
        self._buffer[-1].write(code)

        nl_pos = code.rfind('\n')
        if nl_pos != -1 and code[nl_pos:].strip() == '':
            self._indent_next = True
        elif self._indent_next is True:
            if code.strip() != '':
                self._indent_next = False
        else:
            self._indent_next = True

    def write_line(self, code: str = ""):
        self.write("{}\n".format(code))

    @contextlib.contextmanager
    def block(self,
              line: str,
              *,
              inline: bool = False,
              newline: bool = True) -> None:
        self.write('{} {}'.format(line, '{'))

        if not inline:
            self.write('\n')
            self.push_indent()
        else:
            self.write(' ')

        self._buffer.append(StringIO())

        try:
            yield
        finally:
            text = self._buffer.pop().getvalue()
            if inline:
                text = ' '.join(text.split())
            else:
                self.pop_indent()

            self._buffer[-1].write(text)

            if inline:
                if newline:
                    self._buffer[-1].write(" }\n")
                else:
                    self._indent_next = False
                    self._buffer[-1].write(" } ")
            else:
                if newline:
                    self.write_line("}")
                else:
                    self.write("} ")
                    self._indent_next = False

    def _split_write_statement(self, statement: str) -> None:
        lines = statement.splitlines()
        if len(lines) == 1:
            self.write_line("{};".format(lines[-1]))
        elif len(lines) > 1:
            self.write_line(lines[0].strip())
            # Add hanging indent
            with self.indent():
                for line in lines[1:-1]:
                    self.write_line(line)
                self.write_line("{};".format(lines[-1]))

    def write_code(self, statement: str = '') -> None:
        for stmt in statement.split(';'):
            stmt = stmt.strip()
            if stmt:
                self._split_write_statement(stmt)

    def write_snippet(self, snippet: str) -> None:
        snippet = snippet.replace("\n", "\n" + self.indentation).strip()
        self.write(snippet)
        self.write("\n")

    def comment(self, comment: str) -> None:
        self.write_line('// {}'.format(comment))

    @contextlib.contextmanager
    def label(self, label: str, end: str = '') -> ContextManager:
        self.write_line("{}:".format(label))
        self._buffer.append(StringIO())

        self.push_indent()

        try:
            yield
        finally:
            if end:
                self.write_code(end)

            text = self._buffer.pop().getvalue()

            self.pop_indent()

            self._buffer[-1].write(text)

    @contextlib.contextmanager
    def case(self, *args, end: str = '') -> None:
        for label in args[:-1]:
            self.write_line("case {}:".format(label))
        with self.label("case {}".format(args[-1]), end=end):
            yield

class VectorCppBuilder_NoCHERI(CppBuilder):
    vlen_var: Optional[Tuple[str, VType]]

    def __init__(self):
        super().__init__()
        self.vlen_var = None

    @contextlib.contextmanager
    def preproc_guard(self, name: str):
        self.write_line(f"#if {name}")
        try:
            yield
        finally:
            self.write_line(f"#endif // {name}")

    @contextlib.contextmanager
    def preproc_def_guard(self, name: str):
        self.write_line(f"#ifdef {name}")
        try:
            yield
        finally:
            self.write_line(f"#endif // def {name}")

    @contextlib.contextmanager
    def with_vlen(self, elem_count: str, output_var: str, vtype: VType):
        assert self.vlen_var is None
        self.vlen_var = (output_var, vtype)
        with self.block(""):
            self.write_code(f"size_t {output_var} = {vtype.get_vsetvl_func()}({elem_count})")
            
            try:
                yield
            finally:
                self.vlen_var = None
                pass
    
UNIT_ASM_DEF = "USE_ASM_FOR_UNIT"
STRIDED_ASM_DEF = "USE_ASM_FOR_STRIDED"
INDEXED_ASM_DEF = "USE_ASM_FOR_INDEXED"
MASKED_ASM_DEF = "USE_ASM_FOR_MASKED"
SEGMENTED_ASM_DEF = "USE_ASM_FOR_SEGMENTED"
BYTEMASK_ASM_DEF = "USE_ASM_FOR_BYTEMASK"
FAULTONLYFIRST_ASM_DEF = "USE_ASM_FOR_FAULTONLYFIRST"

ENABLE_UNIT_DEF = "ENABLE_UNIT"
ENABLE_STRIDED_DEF = "ENABLE_STRIDED"
ENABLE_INDEXED_DEF = "ENABLE_INDEXED"
ENABLE_MASKED_DEF = "ENABLE_MASKED"
ENABLE_SEGMENTED_DEF = "ENABLE_SEGMENTED"
ENABLE_BYTEMASK_DEF = "ENABLE_BYTEMASK"
ENABLE_ASM_WHOLEREG_DEF = "ENABLE_ASM_WHOLEREG"
ENABLE_FAULTONLYFIRST_DEF = "ENABLE_FAULTONLYFIRST"

ENABLE_FRAC_LMUL_DEF = "ENABLE_FRAC_LMUL"

PREAMBLE="""// This file was autogenerated by vector_gen.py
// Do not edit!

#include <stdint.h>
#include <riscv_vector.h>

#ifdef __cplusplus
extern "C" {
#endif
void* memset(void* dest, int ch, size_t count) {
    unsigned char ch_uc = (unsigned char)ch;
    unsigned char* dest_uc = (unsigned char*)dest;
    for (int i = 0; i < count; i++) {
        *(dest_uc + i) = ch_uc;
    }

    return dest_uc;
}
#ifdef __cplusplus
}
#endif
""" + f"""
#define ASM_PREG(val) "r"(val)
// GCC doesn't like __has_feature(capabilities), so define a convenience value
// which is only 1 when in LLVM with __has_feature(capabilities)
#define HAS_CAPABILITIES 0

// Patch over differences between GCC, clang, and CHERI-clang
#if defined(__llvm__)
// Clang intrinsics are correct for segmented loads and supports fractional LMUL.
// Clang 14+ has the correct intrinsics for bytemask loads,
// and Clang has been tested with wholereg ASM

    // Use intrinsics for BYTEMASK in newer Clangs, otherwise the intrinsics don't exist
    #if __clang_major__ >= 14
        #define {ENABLE_BYTEMASK_DEF} 1
        #define {BYTEMASK_ASM_DEF} 0
    #else
        // LLVM 13 does not support bytemask
        #define {ENABLE_BYTEMASK_DEF} 0
    #endif

    #if __has_feature(capabilities)
        #undef HAS_CAPABILITIES
        #define HAS_CAPABILITIES 1

        #if __has_feature(pure_capabilities)
            // Replace the ASM pointer register function to use capability register
            #undef ASM_PREG
            #define ASM_PREG(val) "C"(val)
        #endif

        // Enable everything
        #define {ENABLE_UNIT_DEF} 1
        #define {ENABLE_STRIDED_DEF} 1
        #define {ENABLE_INDEXED_DEF} 1
        #define {ENABLE_MASKED_DEF} 1
        #define {ENABLE_SEGMENTED_DEF} 1
        #define {ENABLE_FRAC_LMUL_DEF} 1
        #define {ENABLE_ASM_WHOLEREG_DEF} 1
        #define {ENABLE_FAULTONLYFIRST_DEF} 1
        // BYTEMASK is disabled above

        // Use ASM for everything
        #define {UNIT_ASM_DEF} 1
        #define {STRIDED_ASM_DEF} 1
        #define {INDEXED_ASM_DEF} 1
        #define {MASKED_ASM_DEF} 1
        #define {SEGMENTED_ASM_DEF} 1
        // WHOLEREG is always ASM - there are no whole reg intrinsics
        #define {FAULTONLYFIRST_ASM_DEF} 1
    #else
        // Enable everything
        #define {ENABLE_UNIT_DEF} 1
        #define {ENABLE_STRIDED_DEF} 1
        #define {ENABLE_INDEXED_DEF} 1
        #define {ENABLE_MASKED_DEF} 1
        #define {ENABLE_SEGMENTED_DEF} 1
        #define {ENABLE_FRAC_LMUL_DEF} 1
        #define {ENABLE_ASM_WHOLEREG_DEF} 1
        #define {ENABLE_FAULTONLYFIRST_DEF} 1

        // Use intrinsics for everything
        #define {UNIT_ASM_DEF} 0
        #define {STRIDED_ASM_DEF} 0
        #define {INDEXED_ASM_DEF} 0
        #define {MASKED_ASM_DEF} 0
        #define {SEGMENTED_ASM_DEF} 0
        // Wholereg has no intrinsics, always ASM
        #define {FAULTONLYFIRST_ASM_DEF} 0
    #endif
#elif defined(__GNUC__) && !defined(__INTEL_COMPILER)
// GNU exts enabled, not in LLVM or Intel, => in GCC

// GCC from RISC-V toolchain rvv-intrinsics branch
// (archived at https://github.com/theturboturnip/riscv-gnu-toolchain)
// has incorrect names for segmented intrinsics,
// doesn't support fractional LMUL,
// doesn't support byte-mask,
// emits incorrect code for fault-only-first intrinsics (it seems to emit a vsetvli instruction).

    // Enable everything except fractional LMUL and bytemask
    #define {ENABLE_UNIT_DEF} 1
    #define {ENABLE_STRIDED_DEF} 1
    #define {ENABLE_INDEXED_DEF} 1
    #define {ENABLE_MASKED_DEF} 1
    #define {ENABLE_SEGMENTED_DEF} 1
    #define {ENABLE_FRAC_LMUL_DEF} 0
    #define {ENABLE_BYTEMASK_DEF} 0
    #define {ENABLE_ASM_WHOLEREG_DEF} 1
    #define {ENABLE_FAULTONLYFIRST_DEF} 1

    // Use intrinsics for all except segmented loads
    #define {UNIT_ASM_DEF} 0
    #define {STRIDED_ASM_DEF} 0
    #define {INDEXED_ASM_DEF} 0
    #define {MASKED_ASM_DEF} 0
    #define {SEGMENTED_ASM_DEF} 1
    #define {BYTEMASK_ASM_DEF} 0 // bytemask is disabled
    // Wholereg is always ASM
    #define {FAULTONLYFIRST_ASM_DEF} 1 // fault-only-first intrinsics emit the wrong instruction
#endif

volatile extern int64_t outputAttempted; // magic output device
volatile extern int64_t outputSucceeded; // magic output device
volatile extern int8_t finished; // magic output device
volatile extern int64_t ramBoundary; // edge of writable memory
"""

@dataclass(frozen=True)
class Test:
    name: str
    required_def: Optional[str]

@dataclass
class ArrayArg:
    arg_t: str
    arg_n: int

@dataclass
class Harness:
    name: str
    test_args: Dict[str, Union[str, ArrayArg]]

    def get_prototype(self):
        arg_types = [
            f"{v.arg_t}[{v.arg_n}]" if isinstance(v, ArrayArg) else v
            for v in self.test_args.values()
        ]
        return f"int64_t {self.name}(void (*memcpy_fn)({', '.join(arg_types)}))"

    def get_test_func_decl(self, test_name: str):
        args = ', '.join([
            f"{t.arg_t} {name}[{t.arg_n}]" if isinstance(t, ArrayArg) else f"{t} {name}" 
            for name,t in self.test_args.items()
        ])
        return f"void {test_name}({args})"

class VectorTestsCpp(VectorCppBuilder_NoCHERI):
    tests: Dict[Test, Harness]
    # Mapping of name -> harness spec
    harnesses: Dict[str, Harness]

    def __init__(self):
        super().__init__()
        self.tests = {}
        self.harnesses = {}

    @contextlib.contextmanager
    def new_harness(self, harness: Harness):
        self.harnesses[harness.name] = harness
        with self.block(harness.get_prototype()):
            try:
                yield
            finally:
                pass

    @contextlib.contextmanager
    def new_test(self, test: Test, harness: Harness):
        if harness is not None and harness.name not in self.harnesses:
            raise RuntimeError(f"Harness {harness} not added")
            
        self.tests[test] = harness

        if harness:
            decl = harness.get_test_func_decl(test.name)
        else:
            decl = f"int64_t {test.name}()"
        
        if test.required_def:
            with self.preproc_guard(test.required_def):
                with self.block(decl):
                    try:
                        yield
                    finally:
                        pass
        else:
            with self.block(decl):
                try:
                    yield
                finally:
                    pass

    @contextlib.contextmanager
    def add_main(self):
        with self.block("int main(void)"):
            try:
                yield
            finally:
                pass

def generate_vanilla_harnesses(b: VectorTestsCpp):
    for width in [8, 16, 32, 64]:
        val_t = f"uint{width}_t"

        harness = Harness(
            name=f"vector_memcpy_harness_{val_t}",
            test_args = {
                "n": "size_t",
                "in": f"const {val_t}* __restrict__",
                "out": f"{val_t}* __restrict__",
            }
        )

        # Create harness
        with b.new_harness(harness):
            b.write_snippet(
f"""{val_t} data[128] = {{0}};
{val_t} out_data[128] = {{0}};

for ({val_t} i = 0; i < 128; i++) {{
    data[i] = i;
}}

// ONLY copy 110 elements
memcpy_fn(110, data, out_data);

// Check the first 110 elements of output are the same
// This ensures that the emulator correctly loaded/stored enough values
for ({val_t} i = 0; i < 110; i++) {{
    if (data[i] != out_data[i]) {{
        return 0;
    }}
}}
// Check that the rest are 0 (the original value)
// This ensures that the emulator didn't store more elements than it should have
for ({val_t} i = 110; i < 128; i++) {{
    if (out_data[i] != 0) {{
        return 0;
    }}
}}
return 1;
""")

def generate_masked_harnesses(b: VectorTestsCpp):
    for width in [8, 16, 32, 64]:
        val_t = f"uint{width}_t"

        harness = Harness(
            name=f"vector_memcpy_masked_harness_{val_t}",
            test_args = {
                "n": "size_t",
                "in": f"const {val_t}* __restrict__",
                "out": f"{val_t}* __restrict__",
            }
        )

        # Create harness
        with b.new_harness(harness):
            b.write_snippet(
f"""{val_t} data[128] = {{0}};
{val_t} out_data[128] = {{0}};
const {val_t} SENTINEL_NOT_WRITTEN = 0xbb;

for ({val_t} i = 0; i < 128; i++) {{
    data[i] = i;
    out_data[i] = SENTINEL_NOT_WRITTEN;
}}

// ONLY copy 110 elements
// For the masked function, this should only copy odd-indexed elements.
memcpy_fn(110, data, out_data);

// Check the first 110 elements of output are the same
// This ensures that the emulator correctly loaded/stored enough values
for ({val_t} i = 0; i < 110; i++) {{
    if ((i & 1) == 1 && data[i] != out_data[i]) {{
        return 0;
    }} else if ((i & 1) == 0 && out_data[i] != SENTINEL_NOT_WRITTEN) {{
        return 0;
    }}
}}
// Check that the rest are all the original value
// This ensures that the emulator didn't store more elements than it should have
for ({val_t} i = 110; i < 128; i++) {{
    if (out_data[i] != SENTINEL_NOT_WRITTEN) {{
        return 0;
    }}
}}
return 1;
""")

def generate_segmented_harnesses(b: VectorTestsCpp):
    for width in [8, 16, 32, 64]:
        val_t = f"uint{width}_t"

        harness = Harness(
            name=f"vector_memcpy_segmented_harness_{val_t}",
            test_args = {
                "n": "size_t",
                "in": f"const {val_t}* __restrict__",
                "out": ArrayArg(f"{val_t}* __restrict__", 4),
            }
        )

        # Create harness
        with b.new_harness(harness):
            b.write_snippet(
f"""{val_t} data[128] = {{0}};
{val_t} out_r[32] = {{0}};
{val_t} out_g[32] = {{0}};
{val_t} out_b[32] = {{0}};
{val_t} out_a[32] = {{0}};

for ({val_t} i = 0; i < 128; i++) {{
    data[i] = i;
}}

{val_t}* out_datas[4] = {{out_r, out_g, out_b, out_a}};


// ONLY copy 104 elements = 26 segments
// For the masked function, this should only copy odd-indexed elements.
memcpy_fn(26, data, out_datas);

// Check the first 104 elements = 26 segments of output are the same
// This ensures that the emulator correctly loaded/stored enough values
for ({val_t} i = 0; i < 26; i++) {{
    if (data[i*4 + 0] != out_r[i]) {{
        return 0;
    }}
    if (data[i*4 + 1] != out_g[i]) {{
        return 0;
    }}
    if (data[i*4 + 2] != out_b[i]) {{
        return 0;
    }}
    if (data[i*4 + 3] != out_a[i]) {{
        return 0;
    }}
}}
// Check that the rest are 0 (the original value)
// This ensures that the emulator didn't store more elements than it should have
for ({val_t} i = 26; i < 32; i++) {{
    if (out_r[i] != 0 || out_g[i] != 0 || out_b[i] != 0 || out_a[i] != 0) {{
        return 0;
    }}
}}
return 1;
""")

def generate_unit_tests(b: VectorTestsCpp, vtypes: List[VType]):
    # Create tests
    for vtype in vtypes:
        test = Test(
            f"vector_memcpy_unit_stride_{vtype.get_code()}",
            required_def = ENABLE_UNIT_DEF + (f" && {ENABLE_FRAC_LMUL_DEF}" if vtype.lmul.is_frac() else "")
        )
        vtype_type = vtype.get_unsigned_type()
        vtype_elem_type = vtype.get_unsigned_elem_type()
        vtype_unit_load = f"vle{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_store = f"vse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_load_asm = f"vle{vtype.sew.value}.v"
        vtype_unit_store_asm = f"vse{vtype.sew.value}.v"
        with b.new_test(test, b.harnesses[f"vector_memcpy_harness_{vtype_elem_type}"]):
            with b.block("while (1)"):
                with b.with_vlen("n", "copied_per_iter", vtype):
                    b.write_code(f"if (copied_per_iter == 0) break;")
                    b.write_code(f"{vtype_type} data;")

                    with b.preproc_guard(UNIT_ASM_DEF):
                        b.write_code(f'asm volatile ("{vtype_unit_load_asm} %0, (%1)" : "=vr"(data) : ASM_PREG(in));')
                        b.write_code(f'asm volatile ("{vtype_unit_store_asm} %0, (%1)" :: "vr"(data),  ASM_PREG(out));')
                        b.write_line("#else")
                        b.write_code(f"data = {vtype_unit_load}(in, copied_per_iter);")
                        b.write_code(f"{vtype_unit_store}(out, data, copied_per_iter)")

                    b.write_code(f"in += copied_per_iter;")
                    b.write_code(f"out += copied_per_iter;")
                    b.write_code(f"n -= copied_per_iter;")

def generate_strided_tests(b: VectorTestsCpp, vtypes: List[VType]):
    # Create tests
    for vtype in vtypes:
        test = Test(
            f"vector_memcpy_strided_{vtype.get_code()}",
            required_def = ENABLE_STRIDED_DEF + (f" && {ENABLE_FRAC_LMUL_DEF}" if vtype.lmul.is_frac() else "")
        )
        vtype_type = vtype.get_unsigned_type()
        vtype_elem_type = vtype.get_unsigned_elem_type()
        vtype_unit_load = f"vle{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_store = f"vse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_load_asm = f"vle{vtype.sew.value}.v"
        vtype_unit_store_asm = f"vse{vtype.sew.value}.v"
        vtype_strided_load = f"vlse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_strided_store = f"vsse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_strided_load_asm = f"vlse{vtype.sew.value}.v"
        vtype_strided_store_asm = f"vsse{vtype.sew.value}.v"
        with b.new_test(test, b.harnesses[f"vector_memcpy_harness_{vtype_elem_type}"]):
            b.write_code(f"const size_t STRIDE_ELEMS = 4;")
            b.write_code(f"const size_t STRIDE_BYTES = 4 * sizeof({vtype_elem_type});")
            with b.block("while (1)"):
                with b.with_vlen("n", "copied_per_iter", vtype):
                    b.write_code(f"if (copied_per_iter == 0) break;")
                    b.write_code(f"{vtype_type} data;")

                    # If we have room to do so, copy STRIDE*elems by copying STRIDE vectors each of length `elems`
                    with b.block("if (copied_per_iter * STRIDE_ELEMS < n)"):
                        with b.block("for (size_t i = 0; i < STRIDE_ELEMS; i++)"):
                            b.write_code(f"const {vtype_elem_type}* in_offset = in + i;")
                            b.write_code(f"{vtype_elem_type}* out_offset = out + i;")

                            with b.preproc_guard(STRIDED_ASM_DEF):
                                b.write_code(f'asm volatile ("{vtype_strided_load_asm} %0, (%1), %2" : "=vr"(data) : ASM_PREG(in_offset), "r"(STRIDE_BYTES));')
                                b.write_code(f'asm volatile ("{vtype_strided_store_asm} %0, (%1), %2" :: "vr"(data),  ASM_PREG(out_offset), "r"(STRIDE_BYTES));')
                                b.write_line("#else")
                                b.write_code(f"data = {vtype_strided_load}(in_offset, STRIDE_BYTES, copied_per_iter);")
                                b.write_code(f"{vtype_strided_store}(out_offset, STRIDE_BYTES, data, copied_per_iter)")

                        b.write_code(f"in += copied_per_iter * STRIDE_ELEMS;")
                        b.write_code(f"out += copied_per_iter * STRIDE_ELEMS;")
                        b.write_code(f"n -= copied_per_iter * STRIDE_ELEMS;")
                    with b.block("else"):
                        # We don't have room to do STRIDE*elems, pick up the rest with normal copies
                        with b.preproc_guard(UNIT_ASM_DEF):
                            b.write_code(f'asm volatile ("{vtype_unit_load_asm} %0, (%1)" : "=vr"(data) : ASM_PREG(in));')
                            b.write_code(f'asm volatile ("{vtype_unit_store_asm} %0, (%1)" :: "vr"(data),  ASM_PREG(out));')
                            b.write_line("#else")
                            b.write_code(f"data = {vtype_unit_load}(in, copied_per_iter);")
                            b.write_code(f"{vtype_unit_store}(out, data, copied_per_iter)")

                        b.write_code(f"in += copied_per_iter;")
                        b.write_code(f"out += copied_per_iter;")
                        b.write_code(f"n -= copied_per_iter;")

def generate_indexed_tests(b: VectorTestsCpp, vtypes: List[VType]):
    # Create tests
    # NOTE - these tests do not test different index and element formats.
    # They are both assumed to be the same vtype.
    for vtype in vtypes:
        test = Test(
            f"vector_memcpy_indexed_{vtype.get_code()}",
            required_def = ENABLE_INDEXED_DEF + (f" && {ENABLE_FRAC_LMUL_DEF}" if vtype.lmul.is_frac() else "")
        )
        vtype_type = vtype.get_unsigned_type()
        vtype_elem_type = vtype.get_unsigned_elem_type()
        vtype_unit_load = f"vle{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_store = f"vse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_load_asm = f"vle{vtype.sew.value}.v"
        vtype_unit_store_asm = f"vse{vtype.sew.value}.v"
        vtype_indexed_load = f"vluxei{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_indexed_store = f"vsuxei{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_indexed_load_asm = f"vluxei{vtype.sew.value}.v"
        vtype_indexed_store_asm = f"vsuxei{vtype.sew.value}.v"
        with b.new_test(test, b.harnesses[f"vector_memcpy_harness_{vtype_elem_type}"]):
            b.write_code(f"const size_t ELEM_WIDTH = sizeof({vtype_elem_type});")
            b.write_code(f"const size_t VLMAX = vsetvlmax_e{vtype.sew.value}{vtype.lmul.get_code()}();")
            # Generate indices
            # If vector is 128-bits long, max elements = VLEN/8bits per elem * 8 registers per group = VLEN = 128
            b.write_code(f"{vtype_elem_type} indices[128] = {{0}};")
            with b.block("for (size_t i = 0; i < VLMAX; i++)"):
                # Use XOR to generate a shuffled index pattern
                # Multiply by ELEM_WIDTH because indices should be in terms of bytes
                b.write_code(f"indices[i] = ((({vtype_elem_type}) i) ^ 1) * ELEM_WIDTH")
            # Load indices into a vector
            b.write_code(f"{vtype_type} indices_v;")
            b.write_line("#if HAS_CAPABILITIES")
            b.write_code(f'asm volatile ("{vtype_unit_load_asm} %0, (%1)" : "=vr"(indices_v) : ASM_PREG(indices));')
            b.write_line("#else")
            b.write_code(f"indices_v = {vtype_unit_load}(indices, VLMAX);")
            b.write_line("#endif")

            # Do the memcpy
            with b.block("while (1)"):
                with b.with_vlen("n", "copied_per_iter", vtype):
                    b.write_code(f"if (copied_per_iter == 0) break;")
                    b.write_code(f"{vtype_type} data;")

                    # We can only use indices_v if we're copying a full vector,
                    # because it's shuffled.
                    # Just using [0-copied_per_iter] won't necessarily cover all 0..copied_per_iter-1 values.
                    with b.block("if (copied_per_iter == VLMAX)"):
                        with b.preproc_guard(INDEXED_ASM_DEF):
                            b.write_code(f'asm volatile ("{vtype_indexed_load_asm} %0, (%1), %2" : "=vr"(data) : ASM_PREG(in), "vr"(indices_v));')
                            b.write_code(f'asm volatile ("{vtype_indexed_store_asm} %0, (%1), %2" :: "vr"(data),  ASM_PREG(out), "vr"(indices_v));')
                            b.write_line("#else")
                            b.write_code(f"data = {vtype_indexed_load}(in, indices_v, copied_per_iter);")
                            b.write_code(f"{vtype_indexed_store}(out, indices_v, data, copied_per_iter)")
                    with b.block("else"):
                        with b.preproc_guard(UNIT_ASM_DEF):
                            b.write_code(f'asm volatile ("{vtype_unit_load_asm} %0, (%1)" : "=vr"(data) : ASM_PREG(in));')
                            b.write_code(f'asm volatile ("{vtype_unit_store_asm} %0, (%1)" :: "vr"(data),  ASM_PREG(out));')
                            b.write_line("#else")
                            b.write_code(f"data = {vtype_unit_load}(in, copied_per_iter);")
                            b.write_code(f"{vtype_unit_store}(out, data, copied_per_iter)")

                    b.write_code(f"in += copied_per_iter;")
                    b.write_code(f"out += copied_per_iter;")
                    b.write_code(f"n -= copied_per_iter;")

def generate_masked_tests(b: VectorTestsCpp, vtypes: List[VType]):
    # Create tests
    for vtype in vtypes:
        test = Test(
            f"vector_memcpy_masked_{vtype.get_code()}",
            required_def = ENABLE_MASKED_DEF + (f" && {ENABLE_FRAC_LMUL_DEF}" if vtype.lmul.is_frac() else "")
        )
        vtype_type = vtype.get_unsigned_type()
        vtype_elem_type = vtype.get_unsigned_elem_type()
        vtype_mask_type = vtype.get_mask_type()
        vtype_sew_lmul_ratio = vtype.get_sew_lmul_ratio()
        if vtype_sew_lmul_ratio < 1:
            raise RuntimeError(f"vtype {vtype} has a SEW/LMUL ratio smaller than 1, so won't have relevant intrinsics")
        vtype_unit_load = f"vle{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_load_asm = f"vle{vtype.sew.value}.v"
        vtype_unit_store_asm = f"vse{vtype.sew.value}.v"
        vtype_masked_load = f"vle{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}_m"
        vtype_masked_store = f"vse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}_m"
        with b.new_test(test, b.harnesses[f"vector_memcpy_masked_harness_{vtype_elem_type}"]):
            # Generate mask - make an array of values, each 0 or 1, then set the mask register bits based on those
            b.write_code(f"{vtype_elem_type} mask_ints[128] = {{0}}")
            b.write_code(f"const size_t VLMAX = vsetvlmax_e{vtype.sew.value}{vtype.lmul.get_code()}();")
            with b.block("for (size_t i = 0; i < VLMAX; i++)"):
                b.write_code("mask_ints[i] = i & 1")
            # Load mask ints into a vector
            b.write_code(f"{vtype_type} mask_ints_v")
            with b.preproc_guard(UNIT_ASM_DEF):
                b.write_code(f'asm volatile ("{vtype_unit_load_asm} %0, (%1)" : "=vr"(mask_ints_v) : ASM_PREG(&mask_ints[0]));')
                b.write_line("#else")
                b.write_code(f"mask_ints_v = {vtype_unit_load}(mask_ints, VLMAX);")
            # Create a mask from that vector
            # Use the intrinsic on all platforms, it doesn't involve a pointer
            b.write_code(f"{vtype_mask_type} mask = vmseq_vx_u{vtype.sew.value}{vtype.lmul.get_code()}_b{vtype_sew_lmul_ratio}(mask_ints_v, 1, VLMAX);")
            # If we're on a capabilities platform, we don't use masked intrinsics, so the mask may not be moved into v0 automatically.
            # Do it ourselves instead
            with b.preproc_guard(MASKED_ASM_DEF):
                # Set VLEN, vtype to the same vtype as the mask - i.e. as many 8-bit elements that fit into 1 register
                b.write_code("size_t mask_vlen = vsetvlmax_e8m1();")
                b.write_code(f'asm volatile ("vmv.v.v v0, %0" :: "vr"(mask));')

            with b.block("while (1)"):
                with b.with_vlen("n", "copied_per_iter", vtype):
                    b.write_code(f"if (copied_per_iter == 0) break;")
                    b.write_code(f"{vtype_type} data;")

                    with b.preproc_guard(MASKED_ASM_DEF):
                        # Masked load = unit load with extra argument
                        b.write_code(f'asm volatile ("{vtype_unit_load_asm} %0, (%1), v0.t" : "=vr"(data) : ASM_PREG(in));')
                        b.write_code(f'asm volatile ("{vtype_unit_store_asm} %0, (%1), v0.t" :: "vr"(data),  ASM_PREG(out));')
                        b.write_line("#else")
                        b.write_code(f"data = {vtype_masked_load}(mask, data, in, copied_per_iter);")
                        b.write_code(f"{vtype_masked_store}(mask, out, data, copied_per_iter)")

                    b.write_code(f"in += copied_per_iter;")
                    b.write_code(f"out += copied_per_iter;")
                    b.write_code(f"n -= copied_per_iter;")


def generate_bytemask_tests(b: VectorTestsCpp, vtypes: List[VType]):
    # Create tests
    for vtype in vtypes:
        test = Test(
            f"vector_memcpy_masked_bytemask_load_{vtype.get_code()}",
            required_def = ENABLE_BYTEMASK_DEF + (f" && {ENABLE_FRAC_LMUL_DEF}" if vtype.lmul.is_frac() else "")
        )
        vtype_type = vtype.get_unsigned_type()
        vtype_elem_type = vtype.get_unsigned_elem_type()
        vtype_mask_type = vtype.get_mask_type()
        vtype_sew_lmul_ratio = vtype.get_sew_lmul_ratio()
        if vtype_sew_lmul_ratio < 1:
            raise RuntimeError(f"vtype {vtype} has a SEW/LMUL ratio smaller than 1, so won't have relevant intrinsics")
        if 128 * vtype.lmul.get_num_regs_consumed() / vtype.sew.value > 64:
            raise RuntimeError(f"vtype {vtype} uses >64 elements for VLEN=128. The test doesn't account for this")
        vtype_unit_load_asm = f"vle{vtype.sew.value}.v"
        vtype_unit_store_asm = f"vse{vtype.sew.value}.v"
        vtype_masked_load = f"vle{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}_m"
        vtype_masked_store = f"vse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}_m"
        with b.new_test(test, b.harnesses[f"vector_memcpy_masked_harness_{vtype_elem_type}"]):
            # Generate mask - make a 64-bit value, set every other bit
            b.write_code(f"uint64_t mask_int = 0")
            b.write_code(f"const size_t VLMAX = vsetvlmax_e{vtype.sew.value}{vtype.lmul.get_code()}();")
            b.write_code(f"if (VLMAX > 64) return;")
            with b.block("for (size_t i = 0; i < VLMAX; i++)"):
                b.write_code("mask_int |= (i & 1) << i;")
            # Load a mask from that integer
            b.write_code(f"{vtype_mask_type} mask")
            with b.preproc_guard(BYTEMASK_ASM_DEF):
                b.write_code(f'asm volatile ("vlm.v %0, (%1)" : "=vr"(mask) : ASM_PREG(&mask_int));')
                b.write_line("#else")
                b.write_code(f"mask = vlm_v_b{vtype_sew_lmul_ratio}(&mask_int, VLMAX);")
            # If we're using ASM for masked instructions, load it into v0 manually
            with b.preproc_guard(MASKED_ASM_DEF):
                # Set VLEN, vtype to the same vtype as the mask - i.e. as many 8-bit elements that fit into 1 register
                b.write_code("size_t mask_vlen = vsetvlmax_e8m1();")
                b.write_code(f'asm volatile ("vmv.v.v v0, %0" :: "vr"(mask));')


            with b.block("while (1)"):
                with b.with_vlen("n", "copied_per_iter", vtype):
                    b.write_code(f"if (copied_per_iter == 0) break;")
                    b.write_code(f"{vtype_type} data;")

                    with b.preproc_guard(MASKED_ASM_DEF):
                        # Masked load = unit load with extra argument
                        b.write_code(f'asm volatile ("{vtype_unit_load_asm} %0, (%1), v0.t" : "=vr"(data) : ASM_PREG(in));')
                        b.write_code(f'asm volatile ("{vtype_unit_store_asm} %0, (%1), v0.t" :: "vr"(data),  ASM_PREG(out));')
                        b.write_line("#else")
                        b.write_code(f"data = {vtype_masked_load}(mask, data, in, copied_per_iter);")
                        b.write_code(f"{vtype_masked_store}(mask, out, data, copied_per_iter)")

                    b.write_code(f"in += copied_per_iter;")
                    b.write_code(f"out += copied_per_iter;")
                    b.write_code(f"n -= copied_per_iter;")

def generate_segmented_tests(b: VectorTestsCpp, vtypes: List[VType]):
    # Create tests
    for vtype in vtypes:
        test = Test(
            f"vector_memcpy_segmented_{vtype.get_code()}",
            required_def = ENABLE_SEGMENTED_DEF + (f" && {ENABLE_FRAC_LMUL_DEF}" if vtype.lmul.is_frac() else "")
        )
        vtype_type = vtype.get_unsigned_type()
        vtype_elem_type = vtype.get_unsigned_elem_type()
        vtype_seg_load = f"vlseg4e{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_store = f"vse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_seg_load_asm = f"vlseg4e{vtype.sew.value}.v"
        vtype_unit_store_asm = f"vse{vtype.sew.value}.v"
        vtype_num_regs = vtype.lmul.get_num_regs_consumed()
        with b.new_test(test, b.harnesses[f"vector_memcpy_segmented_harness_{vtype_elem_type}"]):
            with b.block("while (1)"):
                with b.with_vlen("n", "copied_per_iter", vtype):
                    b.write_code(f"if (copied_per_iter == 0) break;")

                    with b.preproc_guard(SEGMENTED_ASM_DEF):
                        # Under capabilities, we don't have a way to force r,g,b,a to use subsequent vector registers.
                        # Therefore we hardcode the registers - {r,g,b,a} = {v4,5,6,7}
                        b.write_code(f'asm volatile ("{vtype_seg_load_asm} v4, (%0)" :: ASM_PREG(in));')
                        b.write_code(f'asm volatile ("{vtype_unit_store_asm} v{4 + vtype_num_regs * 0}, (%0)" :: ASM_PREG(out[0]));')
                        b.write_code(f'asm volatile ("{vtype_unit_store_asm} v{4 + vtype_num_regs * 1}, (%0)" :: ASM_PREG(out[1]));')
                        b.write_code(f'asm volatile ("{vtype_unit_store_asm} v{4 + vtype_num_regs * 2}, (%0)" :: ASM_PREG(out[2]));')
                        b.write_code(f'asm volatile ("{vtype_unit_store_asm} v{4 + vtype_num_regs * 3}, (%0)" :: ASM_PREG(out[3]));')
                        b.write_line("#else")
                        b.write_code(f"{vtype_type} r, g, b, a;")
                        b.write_code(f"{vtype_seg_load}(&r, &g, &b, &a, in, copied_per_iter);")
                        b.write_code(f"{vtype_unit_store}(out[0], r, copied_per_iter)")
                        b.write_code(f"{vtype_unit_store}(out[1], g, copied_per_iter)")
                        b.write_code(f"{vtype_unit_store}(out[2], b, copied_per_iter)")
                        b.write_code(f"{vtype_unit_store}(out[3], a, copied_per_iter)")

                    b.write_code(f"in += copied_per_iter * 4;")
                    with b.block("for (int i = 0; i < 4; i++)"):
                        b.write_code(f"out[i] += copied_per_iter;")
                    b.write_code(f"n -= copied_per_iter;")

def generate_wholereg_tests(b: VectorTestsCpp, vtypes: List[VType]):
    # Create tests
    for vtype in vtypes:
        test = Test(
            f"vector_memcpy_wholereg_{vtype.get_code()}",
            required_def = ENABLE_ASM_WHOLEREG_DEF
        )

        if vtype.lmul.is_frac():
            raise RuntimeError("Can't do whole-register test on fractional lmul")

        vtype_type = vtype.get_unsigned_type()
        vtype_elem_type = vtype.get_unsigned_elem_type()
        vtype_unit_load = f"vle{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_store = f"vse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_load_asm = f"vle{vtype.sew.value}.v"
        vtype_unit_store_asm = f"vse{vtype.sew.value}.v"

        vtype_wholereg_load_asm = f"vl{vtype.lmul.get_num_regs_consumed()}r.v"
        vtype_wholereg_store_asm = f"vs{vtype.lmul.get_num_regs_consumed()}r.v"

        with b.new_test(test, b.harnesses[f"vector_memcpy_harness_{vtype_elem_type}"]):
            b.write_code(f"const size_t VLMAX = vsetvlmax_e{vtype.sew.value}{vtype.lmul.get_code()}();")

            with b.block("while (1)"):
                with b.with_vlen("n", "copied_per_iter", vtype):
                    b.write_code(f"if (copied_per_iter == 0) break;")
                    b.write_code(f"{vtype_type} data;")

                    with b.block("if (copied_per_iter == VLMAX)"):
                        b.write_code(f'asm volatile ("{vtype_wholereg_load_asm} %0, (%1)" : "=vr"(data) : ASM_PREG(in));')
                        b.write_code(f'asm volatile ("{vtype_wholereg_store_asm} %0, (%1)" :: "vr"(data),  ASM_PREG(out));')
                    with b.block("else"):
                        with b.preproc_guard(UNIT_ASM_DEF):
                            b.write_code(f'asm volatile ("{vtype_unit_load_asm} %0, (%1)" : "=vr"(data) : ASM_PREG(in));')
                            b.write_code(f'asm volatile ("{vtype_unit_store_asm} %0, (%1)" :: "vr"(data),  ASM_PREG(out));')
                            b.write_line("#else")
                            b.write_code(f"data = {vtype_unit_load}(in, copied_per_iter);")
                            b.write_code(f"{vtype_unit_store}(out, data, copied_per_iter)")

                    b.write_code(f"in += copied_per_iter;")
                    b.write_code(f"out += copied_per_iter;")
                    b.write_code(f"n -= copied_per_iter;")

def generate_unit_fof_tests(b: VectorTestsCpp, vtypes: List[VType]):
    # Create tests
    for vtype in vtypes:
        test = Test(
            f"vector_memcpy_unit_stride_faultonlyfirst_{vtype.get_code()}",
            required_def = ENABLE_FAULTONLYFIRST_DEF + (f" && {ENABLE_FRAC_LMUL_DEF}" if vtype.lmul.is_frac() else "")
        )
        vtype_type = vtype.get_unsigned_type()
        vtype_elem_type = vtype.get_unsigned_elem_type()
        vtype_unit_load_fof = f"vle{vtype.sew.value}ff_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_store = f"vse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_load_fof_asm = f"vle{vtype.sew.value}.v"
        vtype_unit_store_asm = f"vse{vtype.sew.value}.v"
        with b.new_test(test, b.harnesses[f"vector_memcpy_harness_{vtype_elem_type}"]):
            with b.block("while (1)"):
                with b.with_vlen("n", "copied_per_iter", vtype):
                    b.write_code(f"if (copied_per_iter == 0) break;")
                    b.write_code(f"{vtype_type} data;")
                    b.write_code(f"size_t new_vl;")

                    with b.preproc_guard(FAULTONLYFIRST_ASM_DEF):
                        b.write_code(f'asm volatile ("{vtype_unit_load_fof_asm} %0, (%1)" : "=vr"(data) : ASM_PREG(in));')
                        b.write_code(f'asm volatile ("csrr %0, vl" : "=r"(new_vl));')
                        b.write_code(f"if (new_vl != copied_per_iter) return;")
                        b.write_code(f'asm volatile ("{vtype_unit_store_asm} %0, (%1)" :: "vr"(data),  ASM_PREG(out));')
                        b.write_line("#else")
                        b.write_code(f"data = {vtype_unit_load_fof}(in, &new_vl, copied_per_iter);")
                        b.write_code(f"if (new_vl != copied_per_iter) return;")
                        b.write_code(f"{vtype_unit_store}(out, data, copied_per_iter)")

                    b.write_code(f"in += copied_per_iter;")
                    b.write_code(f"out += copied_per_iter;")
                    b.write_code(f"n -= copied_per_iter;")

def generate_boundary_fof_tests(b: VectorTestsCpp, vtypes: List[VType]):
    # Create tests
    for vtype in vtypes:
        test = Test(
            f"vector_memcpy_boundary_faultonlyfirst_{vtype.get_code()}",
            required_def = ENABLE_FAULTONLYFIRST_DEF + (f" && {ENABLE_FRAC_LMUL_DEF}" if vtype.lmul.is_frac() else "")
        )
        vtype_elem_type = vtype.get_unsigned_elem_type()
        vtype_unit_load_fof = f"vle{vtype.sew.value}ff_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_load_fof_asm = f"vle{vtype.sew.value}ff.v"
        with b.new_test(test, None):
            b.write_code(f"{vtype_elem_type}* unmapped_ptr = ({vtype_elem_type}*)&ramBoundary")

            # Find the number of elements in a single vector register group
            b.write_code(f"const size_t VLMAX = vsetvlmax_e{vtype.sew.value}{vtype.lmul.get_code()}();")

            # Write values to the edge of memory
            with b.block("for (size_t i = 0; i < VLMAX; i++)"):
                b.write_code("*(unmapped_ptr - VLMAX + i) = i")

            # Foreach N in [1, vlmax]
            #     run a test case that reads N elements before hitting the edge of memory
            #     assert the resulting vlen = N
            with b.block("for (size_t expected_num_copied = 1; expected_num_copied <= VLMAX; expected_num_copied++)"):
                b.write_code(f"const {vtype_elem_type}* in = unmapped_ptr - expected_num_copied")
                # Reset the length
                b.write_code(f"vsetvlmax_e{vtype.sew.value}{vtype.lmul.get_code()}();")
                # See how the length changes
                b.write_code(f"size_t new_vl;")
                with b.preproc_guard(FAULTONLYFIRST_ASM_DEF):
                    b.write_code(f'asm volatile ("{vtype_unit_load_fof_asm} v8, (%0)" :: ASM_PREG(in));')
                    b.write_code(f'asm volatile ("csrr %0, vl" : "=r"(new_vl));')
                    b.write_line("#else")
                    b.write_code(f"{vtype_unit_load_fof}(in, &new_vl, VLMAX);")
                b.write_code(f"if (new_vl != expected_num_copied) return 0;")
            b.write_code("return 1")

def generate_tests() -> Tuple[str, Dict[Any, Any]]:
    b = VectorTestsCpp()

    # Create harnesses
    generate_vanilla_harnesses(b)
    generate_masked_harnesses(b)
    generate_segmented_harnesses(b)

    # Create tests
    vtypes = [
        VType(Sew.e8, Lmul.e1),
        VType(Sew.e16, Lmul.e2),
        VType(Sew.e32, Lmul.e4),
        VType(Sew.e64, Lmul.e8),
        # Test fractional lmul
        VType(Sew.e32, Lmul.eHalf),
        VType(Sew.e16, Lmul.eQuarter),
        VType(Sew.e8, Lmul.eEighth),
    ]
    generate_unit_tests(b, vtypes)
    generate_strided_tests(b, vtypes)
    generate_indexed_tests(b, vtypes)
    generate_masked_tests(b, vtypes)
    generate_bytemask_tests(b, vtypes)
    # Can't use m8 for 4x segmented loads
    # At most can use 2x, so that total number of registers = 8
    generate_segmented_tests(b, [
        VType(Sew.e8, Lmul.e2),
        VType(Sew.e16, Lmul.e2),
        VType(Sew.e32, Lmul.e2),
        VType(Sew.e64, Lmul.e2),
        # Test fractional lmul
        VType(Sew.e32, Lmul.eHalf),
    ])
    generate_wholereg_tests(b, [
        VType(Sew.e64, Lmul.e1),
        VType(Sew.e64, Lmul.e2),
        VType(Sew.e64, Lmul.e4),
        VType(Sew.e64, Lmul.e8),
    ])
    generate_unit_fof_tests(b, vtypes)
    generate_boundary_fof_tests(b, vtypes)

    test_json = {}

    # Make main
    b.write_line("")
    b.write_line("#ifdef __cplusplus")
    b.write_code('extern "C" {')
    b.write_line("#endif // __cplusplus")
    with b.add_main():
        b.write_code("int64_t attempted = 0;")
        b.write_code("int64_t successful = 0;")
        b.write_code("")
        for i, (test, harness) in enumerate(b.tests.items()):
            if test.required_def:
                b.write_line(f"#if {test.required_def}")
            b.write_code(f"attempted  |= 1ll << {i};")
            if harness:
                b.write_code(f"successful |= {harness.name}({test.name}) << {i};")
            else:
                b.write_code(f"successful |= {test.name}() << {i};")
            if test.required_def:
                b.write_line(f"#endif // {test.required_def}")
            b.write_line("")

            test_json[i] = {"test": test.name}
        b.write_code("outputAttempted = attempted;")
        b.write_code("outputSucceeded = successful;")
        b.write_code("finished = 1;")
        b.write_code("return 0;")
    b.write_line("#ifdef __cplusplus")
    b.write_code('}')
    b.write_line("#endif // __cplusplus")

    return PREAMBLE + b.get_value(), test_json

if __name__ == '__main__':
    parser = argparse.ArgumentParser("vector_gen", description="Generator for vector test code")
    parser.add_argument("output_cpp", type=str)
    parser.add_argument("output_list", type=str)

    args = parser.parse_args()

    tests, test_json = generate_tests()
    with open(args.output_cpp, "w") as f:
        f.write(tests)
    with open(args.output_list, "w") as f:
        import json
        f.write(json.dumps(test_json, indent=4))
