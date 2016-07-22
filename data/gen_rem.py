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

    t_csu = []
    t_csl = []
    t_cst = []
    t_csf = []

    t_cu = []
    t_cl = []
    t_ct = []
    t_cf = []
    t_cfn = []
    t_cfc = []

    def insert(t_, cx, cm):
        if cm is not True:
            t_.append("(%s,%s)" % (xcp(cx), xcp(cm)))
    def inserts(t_, cx, cms):
        if not (cms and cms[0] is True):
            x = ", ".join(map(xcp, cms))
            t_.append("(%s, &[%s])" % (xcp(cx), x))

    for cp in cp_iter():
        cx = cp.codepoint()

        insert(t_csu, cx, cp.case_upper_simple())
        insert(t_csl, cx, cp.case_lower_simple())
        insert(t_cst, cx, cp.case_title_simple())
        insert(t_csf, cx, cp.case_fold_simple())

        inserts(t_cu, cx, cp.case_upper())
        inserts(t_cl, cx, cp.case_lower())
        inserts(t_ct, cx, cp.case_title())
        inserts(t_cf, cx, cp.case_fold())
        inserts(t_cfn, cx, cp.case_fold_nfkc())
        inserts(t_cfc, cx, cp.casefoldclosure_nfkc())

    table("UCD_CASE_SIMP_UP", "[((u8,u8,u8),(u8,u8,u8))]", t_csu, file=f)
    table("UCD_CASE_SIMP_LW", "[((u8,u8,u8),(u8,u8,u8))]", t_csl, file=f)
    table("UCD_CASE_SIMP_TI", "[((u8,u8,u8),(u8,u8,u8))]", t_cst, file=f)
    table("UCD_CASE_SIMP_FD", "[((u8,u8,u8),(u8,u8,u8))]", t_csf, file=f)

    table("UCD_CASE_UP", "[((u8,u8,u8),&'static [(u8,u8,u8)])]", t_cu, file=f)
    table("UCD_CASE_LW", "[((u8,u8,u8),&'static [(u8,u8,u8)])]", t_cl, file=f)
    table("UCD_CASE_TI", "[((u8,u8,u8),&'static [(u8,u8,u8)])]", t_ct, file=f)
    table("UCD_CASE_FD", "[((u8,u8,u8),&'static [(u8,u8,u8)])]", t_cf, file=f)
    table("UCD_CASE_FD_NFKC", "[((u8,u8,u8),&'static [(u8,u8,u8)])]", t_cfn, file=f)
    table("UCD_CASE_FD_CLOS", "[((u8,u8,u8),&'static [(u8,u8,u8)])]", t_cfc, file=f)
