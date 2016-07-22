#!/usr/bin/env python3
from generate import *
from misc import jt_alias, jg_alias, sc_alias, hst_alias

with open(base % "scripts", "w") as f:
    ranges = cprngs_by('join_class', 'join_group', 'script', 'hangul_syll_type')
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
