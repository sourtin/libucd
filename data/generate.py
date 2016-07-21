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
    print("#[derive(Clone,Copy,Eq,PartialEq,Debug)]", file=file)
    print("pub enum %s {" % name, file=file)
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

def transformed(type_sig, transfn, table_name, ranges, default, file=None):
    t_ = []
    del ranges[default]
    for c,rs in ranges.items():
        cn = transfn(c)
        for ri,rj in rs:
            x = xrng(ri,rj,cn)
            t_.append((ri,rj,x))
    t_.sort()
    table(table_name, "[((u8,u8,u8), (u8,u8,u8), %s)]" % type_sig,
            map(lambda x:x[2], t_), file=file)

def enummed(enum_name, aliases, table_name, ranges, default, file=None):
    enum_map = dict(aliases)
    symbols = [x for _,x in aliases]
    enum(enum_name, symbols, file=file)
    transformed(enum_name, lambda c: "%s::%s" % (enum_name, enum_map[c]),
                    table_name, ranges, default, file)

def booled(table_name, ranges, default, file=None):
    # use unit type so works with search and search_range fns unmodified
    transformed("()", lambda b:{not default:"()"}[b],
            table_name, ranges, default, file)

def mapped16(table_name, maps, file=None):
    maps.sort()
    t_ = []
    for x,y in maps:
        assert(x<65536 and y<65536)
        t_.append("(%d,%d)" % (x,y))
    table(table_name, "[(u16,u16)]", t_, file=file)
