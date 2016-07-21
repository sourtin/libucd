#!/usr/bin/env python3
from generate import *
from misc import cat_alias
import re

with open(base % "general", "w") as f:
    ranges = cprngs_by('age', 'category', 'comb_class')

    age = ranges['age']
    del age[None]
    t_age = []
    for a,rs in age.items():
        maj = int(a[0])
        min = int(a[2])
        dat = "(%d,%d)" % (maj,min)
        for ri,rj in rs:
            x = xrng(ri,rj,dat)
            t_age.append((ri,rj,x))
    t_age.sort()
    table("UCD_AGE", "[((u8,u8,u8), (u8,u8,u8), (u8,u8))]",
            map(lambda x:x[2], t_age), file=f)

    blocks = []
    t_blk = []
    for el in db_iter():
        if el.tag == 'block':
            p = el.attrib
            cpb = int(p['first-cp'], 16)
            cpe = int(p['last-cp'], 16)
            name = re.sub('[-\s]+','',p['name'])
            x = xrng(cpb,cpe,"UnicodeBlock::%s"%name)
            blocks.append(name)
            t_blk.append((cpb,cpe,x))
    t_blk.sort()
    enum("UnicodeBlock", blocks, file=f)
    table("UCD_BLOCK", "[((u8,u8,u8), (u8,u8,u8), UnicodeBlock)]",
            map(lambda x:x[2], t_blk), file=f)

    cat_map = dict(cat_alias)
    cats = [x for _,x in cat_alias]
    t_cat = []
    cat = ranges['category']
    del cat['Cn']
    for c,rs in cat.items():
        cn = "UnicodeCategory::%s" % cat_map[c]
        for ri,rj in rs:
            x = xrng(ri,rj,cn)
            t_cat.append((ri,rj,x))
    t_cat.sort()
    enum("UnicodeCategory", cats, file=f)
    table("UCD_CAT", "[((u8,u8,u8), (u8,u8,u8), UnicodeCategory)]",
            map(lambda x:x[2], t_cat), file=f)

    t_ccc = []
    ccc = ranges['comb_class']
    del ccc[0]
    for c,rs in ccc.items():
        cn = "%d" % c
        for ri,rj in rs:
            x = xrng(ri,rj,cn)
            t_ccc .append((ri,rj,x))
    t_ccc.sort()
    table("UCD_COMBCLS", "[((u8,u8,u8), (u8,u8,u8), u8)]",
            map(lambda x:x[2], t_ccc), file=f)
