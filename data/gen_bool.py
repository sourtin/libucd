#!/usr/bin/env python3
from generate import *
# generate the remaining 30 bool properties
frs = ['quick_nfd', 'quick_nfkd']
tss = ['graph_link', 'expanding_nfc', 'case_changes_casefold', 'case_changes_lower',
        'case_changes_upper', 'comp_excl', 'case_changes_title']
trs = ['case_changes_casemap', 'case_is_lower_other', 'expanding_nfd', 'graph_ext',
        'cased', 'id_start_nfkc', 'expanding_nfkc', 'comp_excl_full', 'case_is_lower',
        'case_is_upper', 'ideo_unified', 'patt_syntax', 'case_changes_casefold_nfkc',
        'id_cont', 'case_ignorable', 'graph_base', 'graph_ext_other', 'id_cont_nfkc',
        'expanding_nfkd', 'ideo', 'id_start']

with open(base % "bool", 'w') as f:
    ranges = cprngs_by(*(frs + tss + trs))
    for r in frs:
        tn = "UCD_%s" % r.upper()
        booled(tn, ranges[r], True, file=f)
        print("pub fn %s(self) -> bool { !in_ranges(&%s, self.0) }" % (r, tn))
    print()

    for s in tss:
        tn = "UCD_%s" % s.upper()
        booled_single(tn, ranges[s], False, file=f)
        print("pub fn %s(self) -> bool { in_table(&%s, self.0) }" % (s, tn))
    print()

    for r in trs:
        tn = "UCD_%s" % r.upper()
        booled(tn, ranges[r], False, file=f)
        print("pub fn %s(self) -> bool { in_ranges(&%s, self.0) }" % (r, tn))
    print()
