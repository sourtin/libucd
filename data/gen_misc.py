#!/usr/bin/env python3
from generate import *
from misc import nt_alias, ea_alias, lb_alias

with open(base % "misc", "w") as f:
    ranges = cprngs_by('numeric_type', 'numeric_value', 'linebreak', 'ea_width')

    """numeric value property

    numeric values held by unicode characters range from 1/160 to 10^12,
    and are all rational. to represent this entire range, we can use a
    tuple of i64 for the numerator (\u0F33 === -1/2 so we need negative
    number support), and a u8 for the denominator (largest is 160). we
    also find however that only 142 values (+NaN) are actually used, so
    the minimal size is to split the table into a codepoint lookup table
    and a second lookup table for the actual numeric value.

    note, NaN (most codepoints) is represented as 0/0
    """

    numbers = []
    nvs = ranges['numeric_value']
    numbers = [n for n,_ in nvs.items() if n != (0,0)]
    numbers.sort(key=lambda n:(n[0]/n[1],n[0],n[1]))
    assert(len(numbers) < 256)
    table("UCD_NUMS", "[(i64,u8)]", map(str,numbers), file=f)

    transformed_single("u8", numbers.index, "UCD_NUMVAL", nvs, (0,0), file=f)
    enummed("NumericType", nt_alias, "UCD_NUMTYPE", ranges['numeric_type'], 'None', file=f)
    enummed("EastAsianWidth", ea_alias, "UCD_EAWIDTH", ranges['ea_width'], 'N', file=f)
    enummed("LinebreakClass", lb_alias, "UCD_LB", ranges['linebreak'], 'XX', file=f)
