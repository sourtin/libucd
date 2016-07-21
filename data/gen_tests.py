#!/usr/bin/env python3
from generate import *
from blocks import blocks
from misc import *

def edx(alias, default=None):
    idx = {}
    for i, (n, _) in enumerate(alias):
        idx[n] = '%d' % i
    if default not in idx:
        idx[default] = ''
    return idx

if False:
    with open(base_test % "age", 'w') as f_age, \
         open(base_test % "block", 'w') as f_blk, \
         open(base_test % "cat", 'w') as f_cat, \
         open(base_test % "ccc", 'w') as f_ccc:

        bl_idx = {None: '-'}
        for i, (_,_,block) in enumerate(blocks):
            bl_idx[block] = '%d' % i
        cat_idx = edx(cat_alias)

        for cp in cp_iter():
            # age
            a = cp.age()
            if a is None: print("-", file=f_age)
            else: print(a[0]+a[2], file=f_age)

            # block
            #  caution, assumes that the enum has
            #  retained its indexing, otherwise
            #  test will be corrupted
            print(bl_idx[cp.block()], file=f_blk)

            # category
            print(cat_idx[cp.category()], file=f_cat)

            # canonical combining class
            print('%d' % cp.comb_class(), file=f_ccc)

if False:
    with open(base_test % "bidi-control", 'w') as f_bctl, \
         open(base_test % "bidi-class", 'w') as f_bcls, \
         open(base_test % "bidi-mirrored", 'w') as f_bmrd, \
         open(base_test % "bidi-bratype", 'w') as f_bbt, \
         open(base_test % "bidi-mirror", 'w') as f_bmr, \
         open(base_test % "bidi-paired", 'w') as f_bpb:

        bidicl_idx = edx(bidicl_alias)
        bidipbt = {None: '-', '(': '(', ')': ')'}

        for cp in cp_iter():
            print('%d' % cp.bidi_control(), file=f_bctl)
            print(bidicl_idx[cp.bidi_class()], file=f_bcls)
            print('%d' % cp.bidi_mirrored(), file=f_bmrd)
            print(bidipbt[cp.bidi_bracket_type()], file=f_bbt)

            m = cp.bidi_mirror()
            c = '%d'%m if m is not None else ''
            print(c, file=f_bmr)

            b = cp.bidi_bracket()
            b2 = b if b is not True else cp.codepoint()
            print('%d' % b2, file=f_bpb)

with open(base_test % "numval", 'w') as f_nv, \
     open(base_test % "numtype", 'w') as f_nt:

    nt_idx = edx(nt_alias, 'None')

    for cp in cp_iter():
        print('%d,%d'%cp.numeric_value(), file=f_nv)
        print(nt_idx[cp.numeric_type()], file=f_nt)
