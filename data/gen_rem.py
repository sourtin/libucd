#!/usr/bin/env python3
from generate import *
from misc import tri_alias

with open(base % "rem", 'w') as f:
    ranges = cprngs_by('quick_nfc', 'quick_nfkc')
    
    qa = ranges['quick_nfc']
    qaf = set(derangify(qa[False]))
    qam = set(derangify(qa[None]))

    qb = ranges['quick_nfkc']
    qbf = set(derangify(qb[False]))
    qbm = set(derangify(qb[None]))

    assert(qaf < qbf)
    assert(qam == qbm)

    qbx = rangify(qbf - qaf)

    enummed("Trilean", tri_alias, "UCD_QNFC", qa, True, file=f)
    booled("UCD_QNFKC", {False: qbx, True: []}, True, file=f)
