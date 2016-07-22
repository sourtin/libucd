#!/usr/bin/env python3
from generate import *

ss = ['hex_digit_ascii', 'prepended_concatenation_mark', 'hyphen', 'hex_digit',
        'white', 'logical_order_exception', 'term_sentence', 'dash', 'quot',
        'term_punc', 'extender', 'soft_dotted'] 
rs = ['default_ignorable', 'alpha', 'default_ignorable_other',
        'math_other', 'diacritic', 'math', 'alpha_other']

with open(base % "function", 'w') as f:
    ranges = cprngs_by(*(ss + rs))
    for s in ss:
        tn = "UCD_%s" % s.upper()
        booled_single(tn, ranges[s], False, file=f)
        print("pub fn %s(self) -> bool { in_table(&%s, self.0) }" % (s, tn))
    print()

    for r in rs:
        tn = "UCD_%s" % r.upper()
        booled(tn, ranges[r], False, file=f)
        print("pub fn %s(self) -> bool { in_ranges(&%s, self.0) }" % (r, tn))
    print()
