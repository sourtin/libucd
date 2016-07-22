#!/usr/bin/env python3
from generate import *
from misc import jt_alias, jg_alias, sc_alias, hst_alias, insc_alias, inpc_alias

with open(base % "scripts", "w") as f:
    ranges = cprngs_by('join_class', 'join_group', 'script',
                       'hangul_syll_type', 'jamo_short_name',
                       'indic_category_syll', 'indic_category_pos')

    enummed_single("JoiningGroup", jg_alias, "UCD_JOINGRP",
            ranges['join_group'], 'No_Joining_Group', file=f)
    enummed("JoiningType", jt_alias, "UCD_JOINTYPE",
            ranges['join_class'], 'U', file=f)

    enummed("Script", sc_alias, "UCD_SCRIPT", ranges['script'], 'Zzzz', file=f)
    sc_map = dict(sc_alias)
    t_scx = []
    scf = lambda s: "Script::%s" % sc_map[s]
    for cp in cp_iter():
        cdp = cp.codepoint()
        scr = cp.script()
        scx = cp.script_extensions()
        if len(scx) > 1 or scx[0] != scr:
            y = ", ".join(map(scf, scx))
            x = "(%s, &[%s])" % (xcp(cdp), y)
            t_scx.append(x)
    table("UCD_SCRIPT_MAP", "[&'static [Script]]",
            ["&[Script::%s]" % s for _,s in sc_alias], file=f)
    table("UCD_SCRIPTEXT", "[((u8,u8,u8), &'static [Script])]", t_scx, file=f)

    enum("HangulSyllableType", [x for _,x in hst_alias], file=f)
    # jamo short name
    # note, one character is indistinguishable in the ucd xml file,
    # \u110B has the value of the empty string, but is not none
    t_jsn = []
    jsns = ranges['jamo_short_name']
    jsns[''] = [(4363,4363)]
    for j,rs in jsns.items():
        for ri,rj in rs:
            for c in range(ri,rj+1):
                x = '(%s,"%s")' % (xcp(c),j)
                t_jsn.append((c,x))
    t_jsn.sort()
    table("UCD_JSN", "[((u8,u8,u8), &'static str)]", map(lambda x:x[1], t_jsn), file=f)

    enummed("IndicSyllabicCategory", insc_alias, "UCD_INSC",
            ranges['indic_category_syll'], 'Other', file=f)
    enummed_single("IndicPositionalCategory", inpc_alias, "UCD_INPC",
            ranges['indic_category_pos'], 'NA', file=f)
