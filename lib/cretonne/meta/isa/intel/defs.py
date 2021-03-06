"""
Intel definitions.

Commonly used definitions.
"""
from __future__ import absolute_import
from cdsl.isa import TargetISA, CPUMode
import base.instructions

ISA = TargetISA('intel', [base.instructions.GROUP])

# CPU modes for 32-bit and 64-bit operation.
I32 = CPUMode('I32', ISA)
I64 = CPUMode('I64', ISA)
