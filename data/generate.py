from process import *
import textwrap
base = "../src/tables/%s.rs"
base_test = "../tests/data/%s.txt"

wrapper = textwrap.TextWrapper(
        width=80,
        break_long_words=False,
        break_on_hyphens=False,
        initial_indent='  ',
        subsequent_indent='  ')

def enum(name, vals, file=None):
    print("enum %s {" % name, file=file)
    for line in wrapper.wrap(', '.join(vals)):
        print(line, file=file)
    print("}\n", file=file)

def table(name, type, vals, file=None):
    print("pub static %s: &'static %s = &[" % (name, type), file=file)
    for line in wrapper.wrap(', '.join(vals)):
        print(line, file=file)
    print("];\n", file=file)

def xrng(ri, rj, dat):
    ri3 = ri%256; ri>>=8
    ri2 = ri%256; ri>>=8
    rit = "(%d,%d,%d)" % (ri,ri2,ri3)

    rj3 = rj%256; rj>>=8
    rj2 = rj%256; rj>>=8
    rjt = "(%d,%d,%d)" % (rj,rj2,rj3)

    return "(%s,%s,%s)" % (rit,rjt,dat)
