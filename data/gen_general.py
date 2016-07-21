#!/usr/bin/env python3
from generate import *
from misc import cat_alias
import re

with open(base % "general", "w") as f:
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

    ranges = cprngs_by('age', 'category', 'comb_class')
    def age_transform(a):
        maj = int(a[0])
        min = int(a[2])
        return "(%d,%d)" % (maj,min)
    transformed("(u8,u8)", age_transform, "UCD_AGE", ranges['age'], None, f)
    enummed("UnicodeCategory", cat_alias, "UCD_CAT", ranges['category'], 'Cn', f)
    transformed("u8", str, "UCD_COMBCLS", ranges['comb_class'], 0, f)
