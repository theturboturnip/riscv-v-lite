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
        bytes_per_elem = vtype.sew.value // 8
        with self.block(""):
            self.write_code(f"size_t {output_var} = {vtype.get_vsetvl_func()}({elem_count}/{bytes_per_elem})")
            
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


def generate_vanilla_tests(b: VectorTestsCpp):
    harness = Harness(
        name="vector_memcpy_harness",
        test_args = {
            "n": "size_t",
            "in": "const int8_t* __restrict__",
            "out": "int8_t* __restrict__",
        }
    )

    # Create harness
    with b.new_harness(harness):
        b.write_code(
"""
uint8_t data[128] = {0};
uint8_t out_data[128] = {0};

for (uint32_t i = 0; i < 128; i++) {
    data[i] = i;
}

// ONLY copy 110 bytes
// Assume vectors are 32/64 bytes
// We want to force memcpy_fn to copy with a not-full vector register, to test vlen
// regardless of what element width (1byte, 2byte, 4byte, 8byte) it uses
// => choose a value that isn't a multiple of 32,64
// = 110
memcpy_fn(110, data, out_data);

// Check the first 110 bytes of output are the same
// This ensures that the emulator correctly loaded/stored enough values
for (uint32_t i = 0; i < 110; i++) {
    if (data[i] != out_data[i]) {
        return 0;
    }
}
// Check that the rest are 0 (the original value)
// This ensures that the emulator didn't store more elements than it should have
for (uint32_t i = 110; i < 128; i++) {
    if (out_data[i] != 0) {
        return 0;
    }
}
return 1;
""")

    # Create tests
    for vtype in VType.iterate():
        test = Test(
            f"vector_memcpy_unit_stride_{vtype.get_code()}",
            required_def = None
        )
        vtype_type = vtype.get_unsigned_type()
        vtype_unit_load = f"vle{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        vtype_unit_store = f"vse{vtype.sew.value}_v_u{vtype.sew.value}{vtype.lmul.get_code()}"
        with b.new_test(test, harness):
            with b.block("while (true)"):
                with b.with_vlen("n", "copied_per_iter", vtype):
                    b.write_code(f"if (copied_per_iter == 0) break;")
                    b.write_code(f"{vtype_type} data = {vtype_unit_load}(in, copied_per_iter);")
                    b.write_code(f"{vtype_unit_store}(out, data, copied_per_iter)")
                    b.write_code(f"in += copied_per_iter;")
                    b.write_code(f"out += copied_per_iter;")
                    b.write_code(f"n -= copied_per_iter;")
            b.write_code(f"// Cleanup, in case 'n' wasn't a clean multiple of {vtype.sew.value//8} bytes")
            with b.block(f"while (n > 0)"):
                b.write_code(f"*out = *in;")
                b.write_code(f"out += 1;")
                b.write_code(f"in += 1;")
                b.write_code(f"n -= 1;")


def generate_tests() -> str:
    b = VectorTestsCpp()

    # Create harnesses+tests
    generate_vanilla_tests(b)

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
