#! /usr/bin/env python

# Using a type in one of its methods is only supported on 3.10+
from __future__ import annotations

from enum import Enum, auto
from dataclasses import dataclass
from typing import List


class Type(Enum):
    Void = auto()
    Int = auto()
    IntPointer = auto()
    Double = auto()
    DoublePointer = auto()

    def to_rust(self) -> str:
        if self == Type.Void:
            return "()"
        elif self == Type.Double:
            return "f64"
        elif self == Type.DoublePointer:
            return "*mut f64"
        elif self == Type.Int:
            return "::libc::c_int"
        elif self == Type.IntPointer:
            return "*mut ::libc::c_int"
        else:
            raise ValueError("Type not supported")

@dataclass
class Arg:
    name: str
    type_: Type

    @staticmethod
    def from_str(inp: str) -> Arg:
        inp = inp.strip()
        type_, name = inp.split()
        if type_ == "double":
            final_type = Type.Double
        elif type_ == "void":
            final_type = Type.Void
        elif type_ == "int":
            final_type = Type.Int
        else:
            raise NotImplementedError(f"Type not supported: {type_}")

        if name.startswith("*"):
            if final_type == Type.Double:
                final_type = Type.DoublePointer
            elif final_type == Type.Int:
                final_type = Type.IntPointer
            name = name[1:]
        return Arg(name, final_type)

    def to_rust(self) -> str:
        return f"{self.name}: {self.type_.to_rust()}"


@dataclass
class Function:
    name: str
    return_type: Type
    args: List[Arg]

    @staticmethod
    def from_decl(full_decl: str) -> Function:
        decl_string, params_string = full_decl.split("(")
        decl = Arg.from_str(decl_string)
        params = [Arg.from_str(p) for p in params_string.split(")")[0].split(",")]
        return Function(decl.name, decl.type_, params)

    def to_rust(self) -> str:
        params = ",".join(arg.to_rust() for arg in self.args)
        return_type = ""
        if self.return_type != Type.Void:
            return_type = f"-> {self.return_type.to_rust()}"

        return f"""
#[no_mangle]
pub unsafe extern "C" fn {self.name}({params}) {return_type} {{
    unimplemented!()
}}
"""

def main() -> None:
    functions = []
    with open("gswteos-10.h", "r") as header:
        current_line: List[str] = []
        for line in header:
            line = line.strip()
            if line.startswith("extern") and not 'extern "C"' in line:
                # new function, deal with previous one
                line = line.strip("extern ")
                if current_line:
                    full_decl = "".join(current_line)
                    functions.append(Function.from_decl(full_decl))
                current_line = [line.strip()]
            elif not line.startswith("extern") and current_line:
                # continuation of a function decl
                current_line += line

        if current_line:
            full_decl = "".join(current_line)
            functions.append(Function.from_decl(full_decl))

    for func in functions:
        print(func.to_rust())

if __name__ == "__main__":
    main()
