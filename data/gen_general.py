#!/usr/bin/env python3
from generate import *
import re

blocks = []
blrng = []

with open(base % "general", "w") as f:
    for el in db_iter():
        if el.tag == 'block':
            p = el.attrib
            cpb = int(p['first-cp'], 16)
            cpe = int(p['last-cp'], 16)
            name = re.sub('[-\s]+','',p['name'])
            x = xrng(cpb,cpe,"UnicodeBlock::%s"%name)
            blocks.append(name)
            blrng.append((cpb,cpe,x))
    blrng.sort()
    enum("UnicodeBlock", blocks, file=f)
    table("UCD_BLOCK", "[((u8,u8,u8), (u8,u8,u8), UnicodeBlock)]",
            map(lambda x:x[2], blrng), file=f)

    ranges = cprngs_by('age')

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
