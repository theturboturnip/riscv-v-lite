#!/usr/bin/env python3.8

import argparse
import contextlib
from dataclasses import dataclass
from enum import Enum
from io import StringIO
from typing import ContextManager, Dict, List, Optional, Tuple

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
        self.write_code(f"#if {name}")
        try:
            yield
        finally:
            self.write_code(f"#endif // {name}")

    @contextlib.contextmanager
    def preproc_def_guard(self, name: str):
        self.write_code(f"#ifdef {name}")
        try:
            yield
        finally:
            self.write_code(f"#endif // def {name}")

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
    
PREAMBLE="""
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
"""

@dataclass(frozen=True)
class Test:
    name: str
    required_def: Optional[str]

@dataclass
class Harness:
    name: str
    test_args: Dict[str, str]

    def get_prototype(self):
        arg_types = list(self.test_args.values())
        return f"int {self.name}(void (*memcpy_fn)({', '.join(arg_types)}))"

    def get_test_func_decl(self, test_name: str):
        args = ', '.join([f"{t} {name}" for name,t in self.test_args.items()])
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
        if harness.name not in self.harnesses:
            raise RuntimeError(f"Harness {harness} not added")
            
        self.tests[test] = harness
        
        if test.required_def:
            with self.preproc_guard(test.required_def):
                with self.block(harness.get_test_func_decl(test.name)):
                    try:
                        yield
                    finally:
                        pass
        else:
            with self.block(harness.get_test_func_decl(test.name)):
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

def generate_unit_tests(b: VectorTestsCpp, vtypes: List[VType]):
    # Create tests
    for vtype in vtypes:
        test = Test(
            f"vector_memcpy_unit_stride_{vtype.get_code()}",
            required_def = None
        )
        vtype_type = vtype.get_unsigned_type()
        vtype_elem_type = vtype.get_unsigned_elem_type()
        vtype_unit_load = f"vle{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_store = f"vse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_load_asm = f"vle{vtype.sew.value}.v"
        vtype_unit_store_asm = f"vse{vtype.sew.value}.v"
        with b.new_test(test, b.harnesses[f"vector_memcpy_harness_{vtype_elem_type}"]):
            with b.block("while (true)"):
                with b.with_vlen("n", "copied_per_iter", vtype):
                    b.write_code(f"if (copied_per_iter == 0) break;")
                    b.write_code(f"{vtype_type} data;")

                    b.write_code("#if __has_feature(capabilities)")
                    b.write_code(f'asm volatile ("{vtype_unit_load_asm} %0, (%1)" : "=vr"(data) : "C"(in));')
                    b.write_code(f'asm volatile ("{vtype_unit_store_asm} %0, (%1)" :: "vr"(data),  "C"(out));')
                    b.write_code("#else")
                    b.write_code(f"data = {vtype_unit_load}(in, copied_per_iter);")
                    b.write_code(f"{vtype_unit_store}(out, data, copied_per_iter)")
                    b.write_code("#endif")

                    b.write_code(f"in += copied_per_iter;")
                    b.write_code(f"out += copied_per_iter;")
                    b.write_code(f"n -= copied_per_iter;")

def generate_strided_tests(b: VectorTestsCpp, vtypes: List[VType]):
    # Create tests
    for vtype in vtypes:
        test = Test(
            f"vector_memcpy_strided_{vtype.get_code()}",
            required_def = None
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
            with b.block("while (true)"):
                with b.with_vlen("n", "copied_per_iter", vtype):
                    b.write_code(f"if (copied_per_iter == 0) break;")
                    b.write_code(f"{vtype_type} data;")

                    # If we have room to do so, copy STRIDE*elems by copying STRIDE vectors each of length `elems`
                    with b.block("if (copied_per_iter * STRIDE_ELEMS < n)"):
                        with b.block("for (size_t i = 0; i < STRIDE_ELEMS; i++)"):
                            b.write_code(f"const {vtype_elem_type}* in_offset = in + i;")
                            b.write_code(f"{vtype_elem_type}* out_offset = out + i;")

                            b.write_code("#if __has_feature(capabilities)")
                            b.write_code(f'asm volatile ("{vtype_strided_load_asm} %0, (%1), %2" : "=vr"(data) : "C"(in_offset), "r"(STRIDE_BYTES));')
                            b.write_code(f'asm volatile ("{vtype_strided_store_asm} %0, (%1), %2" :: "vr"(data),  "C"(out_offset), "r"(STRIDE_BYTES));')
                            b.write_code("#else")
                            b.write_code(f"data = {vtype_strided_load}(in_offset, STRIDE_BYTES, copied_per_iter);")
                            b.write_code(f"{vtype_strided_store}(out_offset, STRIDE_BYTES, data, copied_per_iter)")
                            b.write_code("#endif")

                        b.write_code(f"in += copied_per_iter * STRIDE_ELEMS;")
                        b.write_code(f"out += copied_per_iter * STRIDE_ELEMS;")
                        b.write_code(f"n -= copied_per_iter * STRIDE_ELEMS;")
                    with b.block("else"):
                        # We don't have room to do STRIDE*elems, pick up the rest with normal copies
                        b.write_code("#if __has_feature(capabilities)")
                        b.write_code(f'asm volatile ("{vtype_unit_load_asm} %0, (%1)" : "=vr"(data) : "C"(in));')
                        b.write_code(f'asm volatile ("{vtype_unit_store_asm} %0, (%1)" :: "vr"(data),  "C"(out));')
                        b.write_code("#else")
                        b.write_code(f"data = {vtype_unit_load}(in, copied_per_iter);")
                        b.write_code(f"{vtype_unit_store}(out, data, copied_per_iter)")
                        b.write_code("#endif")

                        b.write_code(f"in += copied_per_iter;")
                        b.write_code(f"out += copied_per_iter;")
                        b.write_code(f"n -= copied_per_iter;")


def generate_tests() -> str:
    b = VectorTestsCpp()

    # Create harnesses+tests
    generate_vanilla_harnesses(b)
    generate_unit_tests(b, [
        VType(Sew.e8, Lmul.e8),
        VType(Sew.e16, Lmul.e8),
        VType(Sew.e32, Lmul.e8),
        # Test fractional lmul
        VType(Sew.e32, Lmul.eHalf),
    ])
    generate_strided_tests(b, [
        VType(Sew.e8, Lmul.e8),
        VType(Sew.e16, Lmul.e8),
        VType(Sew.e32, Lmul.e8),
        # Test fractional lmul
        VType(Sew.e32, Lmul.eHalf),
    ])

    # Make main
    b.write_code("#ifdef __cplusplus")
    b.write_code('extern "C" {')
    b.write_code("#endif // __cplusplus")
    with b.add_main():
        b.write_code("int *outputDevice = (int*) 0xf0000000; // magic output device")
        b.write_code("int result = 0;")
        b.write_code("")
        for i, (test, harness) in enumerate(b.tests.items()):
            b.write_code(f"result |= {harness.name}({test.name}) << {i};")
        b.write_code("")
        b.write_code("outputDevice[0] = result;")
        b.write_code("return result;")
    b.write_code("#ifdef __cplusplus")
    b.write_code('}')
    b.write_code("#endif // __cplusplus")

    return PREAMBLE + b.get_value()

if __name__ == '__main__':
    parser = argparse.ArgumentParser("vector_gen", description="Generator for vector test code")
    parser.add_argument("output_cpp", type=str)

    args = parser.parse_args()

    tests = generate_tests()
    with open(args.output_cpp, "w") as f:
        f.write(tests)
