#!/usr/bin/env python3
import xml.etree.ElementTree as et

from blocks import blocks
blassoc = {}

def _boolean(name):
    bool_decode = {
        'Y': True,
        'N': False
    }

    def method(self):
        return bool_decode[self._p[name]]
    return method

def _booly(name):
    bool_decode = {
        'Y': True,
        'N': False,
        'M': None
    }

    def method(self):
        return bool_decode[self._p[name]]
    return method

def _codepoint(name):
    def method(self):
        cp = self._p[name]
        if not cp: return None
        elif cp == '#': return True
        else: return int(cp, 16)
    return method

def _codepoints(name):
    def method(self):
        cps = self._p[name]
        if cps == '#':
            return [self._c]
        else:
            return [int(cp, 16) for cp in cps.split(' ')]
    return method

def _raw(name):
    def method(self):
        return self._p[name]
    return method

def _raws(name):
    def method(self):
        return self._p[name].split(' ')
    return method

class Codepoint:
    def __init__(self, cp, el):
        self._c = cp
        self._p = el.attrib.copy()
        self._n = []
        for ell in el:
            if ell.tag == "name-alias":
                n = ell.attrib['alias']
                t = ell.attrib['type']
                self._n.append((t,n))

    def codepoint(self):
        return self._c

    def names(self):
        names = []
        if self._p['na']:
            names.append((True, self._p['na']))
        if self._p['na1']:
            names.append((False, self._p['na1']))
        return self._n + names

    def age(self):
        a = self._p['age']
        if a == "unassigned":
            return None
        return a

    def block(self):
        bl = self._p['blk']
        bl_ = None
        cp = self._c
        for b,e,n in blocks:
            if b <= cp <= e:
                bl_ = n
                break
        return bl, bl_

    category = _raw('gc')
    def comb_class(self):
        return int(self._p['ccc'])

    bidi_class = _raw('bc')
    bidi_mirrored = _boolean('bidi_M')
    bidi_mirror = _codepoint('bmg')
    bidi_control = _boolean('bidi_C')
    bidi_bracket = _codepoint('bpb')
    def bidi_bracket_type(self):
        return {
            'n': None,
            'o': '(',
            'c': ')'
        }[self._p['bt']]

    decomp_type = _raw('dt')
    decomp_map = _codepoints('dm')
    comp_excl = _boolean('CE')
    comp_excl_full = _boolean('Comp_Ex')
    quick_nfc = _booly('NFC_QC')
    quick_nfd = _boolean('NFD_QC')
    quick_nfkc = _booly('NFKC_QC')
    quick_nfkd = _boolean('NFKD_QC')

    # deprecated
    expanding_nfc = _boolean('NFC_QC')
    expanding_nfd = _boolean('NFD_QC')
    expanding_nfkc = _boolean('NFKC_QC')
    expanding_nfkd = _boolean('NFKD_QC')
    casefoldclosure_nfkc = _codepoints('FC_NFKC')
    iso_comment = _raw('isc')

    numeric_type = _raw('nt')
    def numeric_value(self):
        nv = self._p['nv']
        if nv != 'NaN':
            n = list(map(int, nv.split('/')))
            if len(n) == 1:
                return n[0], 1
            elif len(n) == 2:
                return n[0], n[1]
        return 0, 0

    join_class = _raw('jt')
    join_group = _raw('jg')
    join_control = _boolean('Join_C')
    libebreak = _raw('lb')
    ea_width = _raw('ea')

    case_is_upper = _boolean('Upper')
    case_is_upper_other = _boolean('OUpper')
    case_is_lower = _boolean('Lower')
    case_is_lower_other = _boolean('OLower')

    case_upper = _codepoints('uc')
    case_upper_simple = _codepoint('suc')
    case_lower = _codepoints('lc')
    case_lower_simple = _codepoint('slc')
    case_title = _codepoints('tc')
    case_title_simple = _codepoint('stc')
    case_fold = _codepoints('cf')
    case_fold_simple = _codepoints('scf')
    case_fold_nfkc = _codepoints('NFKC_CF')

    case_ignorable = _boolean('CI')
    cased = _boolean('Cased')
    case_changes_casefold = _boolean('CWCF')
    case_changes_casefold_nfkc = _boolean('CWKCF')
    case_changes_casemap = _boolean('CWCM')
    case_changes_lower = _boolean('CWL')
    case_changes_upper = _boolean('CWU')
    case_changes_title = _boolean('CWT')

    script = _raw('sc')
    script_extensions = _raws('scx')
    hangul_syll_type = _raw('hst')
    jamo_short_name = _raw('JSN')
    indic_category_syll = _raw('InSC')
    indic_category_matra = _raw('InMC')
    indic_category_pos = _raw('InPC')

    id_start = _boolean('IDS')
    id_start_other = _boolean('OIDS')
    id_start_nfkc = _boolean('XIDS')
    id_cont = _boolean('IDC')
    id_cont_other = _boolean('OIDC')
    id_cont_nfkc = _boolean('XIDC')
    patt_syntax = _boolean('Pat_Syn')
    patt_white = _boolean('Pat_WS')

    dash = _boolean('Dash')
    quot = _boolean('QMark')
    term_punc = _boolean('Term')
    term_sentence = _boolean('STerm')
    diacritic = _boolean('Dia')
    extender = _boolean('Ext')
    prepended_concatenation_mark = _boolean('PCM')
    soft_dotted = _boolean('SD')
    alpha = _boolean('Alpha')
    alpha_other = _boolean('OAlpha')
    math = _boolean('Math')
    math_other = _boolean('OMath')
    hex_digit = _boolean('Hex')
    hex_digit_ascii = _boolean('AHex')
    default_ignorable = _boolean('DI')
    default_ignorable_other = _boolean('ODI')
    logical_order_exception = _boolean('LOE')
    white = _boolean('WSpace')
    # deprecated
    hyphen = _boolean('Hyphen')

    graph_base = _boolean('Gr_Base')
    graph_ext = _boolean('Gr_Ext')
    graph_ext_other = _boolean('OGr_Ext')
    # deprecated
    graph_link = _boolean('Gr_Link')

    break_graph_cluster = _boolean('GCB')
    break_word = _boolean('WB')
    break_sentence = _boolean('SB')

    ideo = _boolean('Ideo')
    ideo_unified = _boolean('UIdeo')
    ideo_desc_seq_bin_op = _boolean('IDSB')
    ideo_desc_seq_trin_op = _boolean('IDST')
    ideo_desc_seq_radical = _boolean('Radical')

    deprecated = _boolean('Dep')
    var_sel = _boolean('VS')
    nonchar = _boolean('NChar')

    # unihan
    # tangut



noch = 0
len1 = 0
len2 = 0

tmp=0
tmp2=set()

for (_, el) in et.iterparse('ucd.xml'):
    if '}' in el.tag:
        el.tag = el.tag.split('}', 1)[1]

    if el.tag in ("char", "reserved", "noncharacter"):
        props = el.attrib

        try:
            if 'cp' in props:
                cp = int(props['cp'], 16)
                ncp = 1
            else:
                cp = (int(props['first-cp'], 16),
                        int(props['last-cp'], 16))
                ncp = cp[1]-cp[0]

            names = []
            for ell in el:
                if ell.tag == "name-alias":
                    n = ell.attrib['alias']
                    t = ell.attrib['type']
                    names.append((n,t))
            if props['na']:
                names.append((props['na'],True))
                len1 += len(props['na'])
            if props['na1']:
                names.append((props['na1'],False))

            nv = props['nv']
            ccc = props['ccc']
            bpb = int(props['bpb'], 16) if props['bpb']!='#' else 0

            if props['scx']!='#':
                s=len(props['scx'].split(' '))
                if s > 1:
                    tmp2.add(props['scx'])
                else:
                    tmp+=ncp

            #print("cp: %x" % cp)
            for n,t in names:
                #print("name[%s]: %s" % (t,n))
                len2 += len(n)
            #print()
            #input()

            noch += 1

        except:
            print(props)

        el.clear()

print(tmp,tmp2,len(tmp2))
print(noch, len1, len2)
