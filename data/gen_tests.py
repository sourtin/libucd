#!/usr/bin/env python3
from generate import *
from blocks import blocks
from misc import *
import re

def edx(alias, default='~~~~~~~~~~~~~~'):
    idx = {}
    for i, (n, _) in enumerate(alias):
        idx[n] = '%d' % i
    idx[default] = ''
    return idx
b2s = {False: '0', True: '1'}

def c2s(cp, cx=''):
    if cp is True:
        return str(cx)
    elif cp is None:
        return ''
    else:
        return str(cp)

def cs2s(cps, cx=''):
    return ' '.join(c2s(cp, cx) for cp in cps)


with open(base_test % "age", 'w') as f_age, \
     open(base_test % "block", 'w') as f_blk, \
     open(base_test % "cat", 'w') as f_cat, \
     open(base_test % "ccc", 'w') as f_ccc:

    bl_idx = {None: '-'}
    for i, (_,_,block) in enumerate(blocks):
        bl_idx[block] = '%d' % i
    cat_idx = edx(cat_alias)

    for cp in cp_iter():
        a = cp.age()
        if a is None: print("-", file=f_age)
        else: print(a[0]+a[2], file=f_age)

        # block
        #  caution, assumes that the enum has
        #  retained its indexing, otherwise
        #  test will be corrupted
        #
        #  unlike the enums defined in misc.py,
        #  the block enums are only guaranteed
        #  valid for v9.0.0, as the actual
        #  coded enum is recalculated directly
        #  from the database, whereas the one
        #  we use here is a hardcoded map from
        #  the blk property, which can't be
        #  robustly calculated
        print(bl_idx[cp.block()], file=f_blk)

        print(cat_idx[cp.category()], file=f_cat)
        print('%d' % cp.comb_class(), file=f_ccc)


with open(base_test % "bidi-control", 'w') as f_bctl, \
     open(base_test % "bidi-class", 'w') as f_bcls, \
     open(base_test % "bidi-mirrored", 'w') as f_bmrd, \
     open(base_test % "bidi-bratype", 'w') as f_bbt, \
     open(base_test % "bidi-mirror", 'w') as f_bmr, \
     open(base_test % "bidi-paired", 'w') as f_bpb:

    bidicl_idx = edx(bidicl_alias)
    bidipbt = {None: '-', '(': '(', ')': ')'}

    for cp in cp_iter():
        print(b2s[cp.bidi_control()], file=f_bctl)
        print(bidicl_idx[cp.bidi_class()], file=f_bcls)
        print(b2s[cp.bidi_mirrored()], file=f_bmrd)
        print(bidipbt[cp.bidi_bracket_type()], file=f_bbt)
        print(c2s(cp.bidi_mirror()), file=f_bmr)
        print(c2s(cp.bidi_bracket(), cp.codepoint()), file=f_bpb)


with open(base_test % "numval", 'w') as f_nv, \
     open(base_test % "numtype", 'w') as f_nt, \
     open(base_test % "eawidth", 'w') as f_ea, \
     open(base_test % "linebreak", 'w') as f_lb, \
     open(base_test % "deprec", 'w') as f_dep, \
     open(base_test % "varsel", 'w') as f_vs, \
     open(base_test % "nonchar", 'w') as f_nc:

    nt_idx = edx(nt_alias, 'None')
    ea_idx = edx(ea_alias)
    lb_idx = edx(lb_alias, 'XX')

    for cp in cp_iter():
        print('%d,%d'%cp.numeric_value(), file=f_nv)
        print(nt_idx[cp.numeric_type()], file=f_nt)
        print(ea_idx[cp.ea_width()], file=f_ea)
        print(lb_idx[cp.linebreak()], file=f_lb)
        print(b2s[cp.deprecated()], file=f_dep)
        print(b2s[cp.var_sel()], file=f_vs)
        print(b2s[cp.nonchar()], file=f_nc)


with open(base_test % "joinctl", 'w') as f_jc, \
     open(base_test % "jointyp", 'w') as f_jt, \
     open(base_test % "joingrp", 'w') as f_jg, \
     open(base_test % "indic-sylcat", 'w') as f_isc, \
     open(base_test % "indic-poscat", 'w') as f_ipc, \
     open(base_test % "hangulst", 'w') as f_hst, \
     open(base_test % "jsn", 'w') as f_jsn:

    jt_idx = edx(jt_alias)
    jg_idx = edx(jg_alias)
    isc_idx = edx(insc_alias)
    ipc_idx = edx(inpc_alias, 'NA')
    hst_idx = edx(hst_alias, 'NA')

    for cp in cp_iter():
        print(b2s[cp.join_control()], file=f_jc)
        print(jt_idx[cp.join_class()], file=f_jt)
        print(jg_idx[cp.join_group()], file=f_jg)
        print(isc_idx[cp.indic_category_syll()], file=f_isc)
        print(ipc_idx[cp.indic_category_pos()], file=f_ipc)
        print(hst_idx[cp.hangul_syll_type()], file=f_hst)

        jsn = cp.jamo_short_name()
        if not jsn and cp.codepoint() != 4363:
            jsn = "-"
        print(jsn, file=f_jsn)

with open(base_test % "script", 'w') as f_sc, \
     open(base_test % "scrext", 'w') as f_scx:

    sc_idx = edx(sc_alias, 'Zzzz')

    for cp in cp_iter():
        scx = cp.script_extensions()
        x = " ".join(sc_idx[s] for s in scx)
        print(sc_idx[cp.script()], file=f_sc)
        print(x, file=f_scx)


ss = ['hex_digit_ascii', 'prepended_concatenation_mark', 'hyphen', 'hex_digit',
        'white', 'logical_order_exception', 'term_sentence', 'dash', 'quot',
        'term_punc', 'extender', 'soft_dotted', 'default_ignorable', 'alpha',
        'default_ignorable_other', 'math_other', 'diacritic', 'math', 'alpha_other']
fs = {s:open(base_test % re.sub('_', '-', s), 'w') for s in ss}
try:
    for cp in cp_iter():
        for s in ss:
            print(b2s[getattr(cp, s)()], file=fs[s])
finally:
    for f in fs.values(): f.close()


ss = ['comp_excl', 'comp_excl_full', 'quick_nfd', 'quick_nfkd', 'expanding_nfc',
        'expanding_nfd', 'expanding_nfkc', 'expanding_nfkd', 'case_is_upper',
        'case_is_upper_other', 'case_is_lower', 'case_is_lower_other',
        'case_ignorable', 'cased', 'case_changes_casefold', 'case_changes_casefold_nfkc',
        'case_changes_casemap', 'case_changes_lower', 'case_changes_upper',
        'case_changes_title', 'id_start', 'id_start_other', 'id_start_nfkc', 'id_cont',
        'id_cont_other', 'id_cont_nfkc', 'patt_syntax', 'patt_white', 'graph_base',
        'graph_ext', 'graph_ext_other', 'graph_link', 'ideo', 'ideo_unified',
        'ideo_desc_seq_bin_op', 'ideo_desc_seq_trin_op', 'ideo_desc_seq_radical']
fs = {s:open(base_test % re.sub('_', '-', s), 'w') for s in ss}
try:
    for cp in cp_iter():
        for s in ss:
            print(b2s[getattr(cp, s)()], file=fs[s])
finally:
    for f in fs.values(): f.close()


with open(base_test % "qnfc", 'w') as f_qnfc, \
     open(base_test % "qnfkc", 'w') as f_qnfkc, \
     open(base_test % "suc", 'w') as f_suc, \
     open(base_test % "slc", 'w') as f_slc, \
     open(base_test % "stc", 'w') as f_stc, \
     open(base_test % "scf", 'w') as f_scf, \
     open(base_test % "uc", 'w') as f_uc, \
     open(base_test % "lc", 'w') as f_lc, \
     open(base_test % "tc", 'w') as f_tc, \
     open(base_test % "cf", 'w') as f_cf, \
     open(base_test % "cf-nfkc", 'w') as f_cfn, \
     open(base_test % "cf-closure", 'w') as f_cfc:

    t2s = edx(tri_alias)

    for cp in cp_iter():
        cx = cp.codepoint()
        print(t2s[cp.quick_nfc()], file=f_qnfc)
        print(t2s[cp.quick_nfkc()], file=f_qnfkc)

        print(c2s(cp.case_upper_simple(), cx), file=f_suc)
        print(c2s(cp.case_lower_simple(), cx), file=f_slc)
        print(c2s(cp.case_title_simple(), cx), file=f_stc)
        print(c2s(cp.case_fold_simple(), cx), file=f_scf)

        print(cs2s(cp.case_upper(), cx), file=f_uc)
        print(cs2s(cp.case_lower(), cx), file=f_lc)
        print(cs2s(cp.case_title(), cx), file=f_tc)
        print(cs2s(cp.case_fold(), cx), file=f_cf)
        print(cs2s(cp.case_fold_nfkc(), cx), file=f_cfn)
        print(cs2s(cp.casefoldclosure_nfkc(), cx), file=f_cfc)


with open(base_test % "dmap", 'w') as f_dm, \
     open(base_test % "dtype", 'w') as f_dt, \
     open(base_test % "wbrk", 'w') as f_wb, \
     open(base_test % "sbrk", 'w') as f_sb, \
     open(base_test % "gcb", 'w') as f_gcb:

    dt_idx = edx(dt_alias, 'none')
    wb_idx = edx(wb_alias)
    sb_idx = edx(sb_alias)
    gcb_idx = edx(gcb_alias)

    for cp in cp_iter():
        cx = cp.codepoint()
        print(cs2s(cp.decomp_map(), cx), file=f_dm)
        print(dt_idx[cp.decomp_type()], file=f_dt)
        print(wb_idx[cp.break_word()], file=f_wb)
        print(sb_idx[cp.break_sentence()], file=f_sb)
        print(gcb_idx[cp.break_graph_cluster()], file=f_gcb)
