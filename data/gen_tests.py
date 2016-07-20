#!/usr/bin/env python3
from generate import *
from blocks import blocks

bl_idx = {None: '-'}
for i, (_,_,block) in enumerate(blocks):
    bl_idx[block] = '%d' % i

with open(base_test % "age", 'w') as f_age, \
     open(base_test % "block", 'w') as f_blk:
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
