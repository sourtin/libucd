#!/usr/bin/env python3
from generate import *
from misc import dt_alias, gcb_alias, wb_alias, sb_alias

with open(base % "decomp", 'w') as f:
    ranges = cprngs_by('decomp_type', 'break_graph_cluster', 'break_word', 'break_sentence')
    gcb = ranges['break_graph_cluster']
    del gcb['L'], gcb['V'], gcb['T'], gcb['LV'], gcb['LVT']

    enummed("DecompositionType", dt_alias, "UCD_DECOMP_TYPE", ranges['decomp_type'], 'none', file=f)
    enummed("GraphemeClusterBreak", gcb_alias, "UCD_GCB", gcb, 'XX', file=f)
    enummed("WordBreak", wb_alias, "UCD_WBRK", ranges['break_word'], 'XX', file=f)
    enummed("SentenceBreak", sb_alias, "UCD_SBRK", ranges['break_sentence'], 'XX', file=f)

    t_dm = []
    for cp in cp_iter():
        inserts(t_dm, cp.codepoint(), cp.decomp_map())
    table("UCD_DECOMP_MAP", "[((u8,u8,u8),&'static [(u8,u8,u8)])]", t_dm, file=f)
