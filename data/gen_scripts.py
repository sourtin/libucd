#!/usr/bin/env python3
from generate import *
from misc import jt_alias, jg_alias

with open(base % "scripts", "w") as f:
    ranges = cprngs_by('join_class', 'join_group')
    enummed_single("JoiningGroup", jg_alias, "UCD_JOINGRP",
            ranges['join_group'], 'No_Joining_Group', file=f)
    enummed("JoiningType", jt_alias, "UCD_JOINTYPE",
            ranges['join_class'], 'U', file=f)
