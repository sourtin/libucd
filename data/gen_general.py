#!/usr/bin/env python3
from generate import *

ranges = cprngs_by('age')

with open(base % "general", "w") as f:
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

with open(base_test % "age", 'w') as f_age:
    for cp in cp_iter():
        a = cp.age()
        if a is None: print("-", file=f_age)
        else: print(a[0]+a[2], file=f_age)
