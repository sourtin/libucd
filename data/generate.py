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

def xcp(cp):
    cp3 = cp%256; cp>>=8
    cp2 = cp%256; cp>>=8
    return "(%d,%d,%d)" % (cp,cp2,cp3)

def xrng(ri, rj, dat):
    return "(%s,%s,%s)" % (xcp(ri),xcp(rj),dat)

def transformed_single(type_sig, transfn, table_name, ranges, default, file=None):
    t_ = []
    del ranges[default]
    for c,rs in ranges.items():
        cn = transfn(c)
        for ri,rj in rs:
            for cp in range(ri,rj+1):
                x = "(%s,%s)" % (xcp(cp), cn)
                t_.append((cp,x))
    t_.sort()
    table(table_name, "[((u8,u8,u8), %s)]" % type_sig,
            map(lambda x:x[1], t_), file=file)
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

def enummed(enum_name, aliases, table_name, ranges, default, file=None, tf=transformed):
    enum_map = dict(aliases)
    symbols = [x for _,x in aliases]
    enum(enum_name, symbols, file=file)
    tf(enum_name, lambda c: "%s::%s" % (enum_name, enum_map[c]),
                    table_name, ranges, default, file)
def enummed_single(*args, **kwargs):
    kwargs['tf'] = transformed_single
    enummed(*args, **kwargs)

def booled(table_name, ranges, default, file=None):
    # use unit type so works with search and search_range fns unmodified
    transformed("()", lambda b:{not default:"()"}[b],
            table_name, ranges, default, file)
def booled_single(table_name, ranges, default, file=None):
    transformed_single("()", lambda b:{not default:"()"}[b],
            table_name, ranges, default, file)

def mapped16(table_name, maps, file=None):
    maps.sort()
    t_ = []
    for x,y in maps:
        assert(x<65536 and y<65536)
        t_.append("(%d,%d)" % (x,y))
    table(table_name, "[(u16,u16)]", t_, file=file)

def insert(t_, cx, cm):
    if cm is not True:
        t_.append("(%s,%s)" % (xcp(cx), xcp(cm)))

def inserts(t_, cx, cms):
    if not (cms and cms[0] is True):
        x = ", ".join(map(xcp, cms))
        t_.append("(%s, &[%s])" % (xcp(cx), x))
