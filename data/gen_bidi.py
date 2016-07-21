#!/usr/bin/env python3
from generate import *
from misc import bidicl_alias

with open(base % "bidi", "w") as f:
    ranges = cprngs_by('bidi_class', 'bidi_mirrored', 'bidi_bracket_type')

    enummed("BidiClass", bidicl_alias, "UCD_BIDI_CLASS", ranges['bidi_class'], 'L', f)
    booled("UCD_BIDI_MIRRORED", ranges['bidi_mirrored'], False, f)
    enummed("BidiPairedBracketType", [('(', 'Open'), (')', 'Close')],
                "UCD_BIDI_BRATYPE", ranges['bidi_bracket_type'], None, f)

    d_mirror = []
    d_paired = []
    for cp in cp_iter():
        c = cp.codepoint()

        mirror = cp.bidi_mirror()
        if mirror is not None:
            d_mirror.append((c,mirror))

        paired = cp.bidi_bracket()
        if paired is not True:
            d_paired.append((c,paired))

    mapped16("UCD_BIDI_MIRROR", d_mirror, f)
    mapped16("UCD_BIDI_PAIRED", d_paired, f)
