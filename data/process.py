#!/usr/bin/env python3
import xml.etree.ElementTree as et
from blocks import blocks_map
import lzma

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
        elif cps:
            return [int(cp, 16) for cp in cps.split(' ')]
        else:
            return []
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

    @staticmethod
    def from_el(el):
        props = el.attrib

        if 'cp' in props:
            cps = (int(props['cp'], 16),) * 2
        else:
            cps = (int(props['first-cp'], 16),
                    int(props['last-cp'], 16))

        cp0, cpn = cps
        for cp in range(cp0, cpn+1):
            yield Codepoint(cp, el)

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
        return blocks_map[self._p['blk']]

    category = _raw('gc')
    def comb_class(self):
        return int(self._p['ccc'])

    bidi_class = _raw('bc')
    bidi_mirrored = _boolean('Bidi_M')
    bidi_mirror = _codepoint('bmg')
    bidi_control = _boolean('Bidi_C')
    bidi_bracket = _codepoint('bpb')
    def bidi_bracket_type(self):
        return {
            'n': None,
            'o': '(',
            'c': ')'
        }[self._p['bpt']]

    decomp_type = _raw('dt')
    decomp_map = _codepoints('dm')
    comp_excl = _boolean('CE')
    comp_excl_full = _boolean('Comp_Ex')
    quick_nfc = _booly('NFC_QC')
    quick_nfd = _boolean('NFD_QC')
    quick_nfkc = _booly('NFKC_QC')
    quick_nfkd = _boolean('NFKD_QC')

    # deprecated
    expanding_nfc = _boolean('XO_NFC')
    expanding_nfd = _boolean('XO_NFD')
    expanding_nfkc = _boolean('XO_NFKC')
    expanding_nfkd = _boolean('XO_NFKD')
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
    #indic_category_matra = _raw('InMC') # removed
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

    break_graph_cluster = _raw('GCB')
    break_word = _raw('WB')
    break_sentence = _raw('SB')

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

    def all(self):
        attrs = ['codepoint', 'names', 'age', 'block', 'category', 'comb_class', 'bidi_class', 'bidi_mirrored', 'bidi_mirror', 'bidi_control', 'bidi_bracket', 'bidi_bracket_type', 'decomp_type', 'decomp_map', 'comp_excl', 'comp_excl_full', 'quick_nfc', 'quick_nfd', 'quick_nfkc', 'quick_nfkd', 'expanding_nfc', 'expanding_nfd', 'expanding_nfkc', 'expanding_nfkd', 'casefoldclosure_nfkc', 'iso_comment', 'numeric_type', 'numeric_value', 'join_class', 'join_group', 'join_control', 'libebreak', 'ea_width', 'case_is_upper', 'case_is_upper_other', 'case_is_lower', 'case_is_lower_other', 'case_upper', 'case_upper_simple', 'case_lower', 'case_lower_simple', 'case_title', 'case_title_simple', 'case_fold', 'case_fold_simple', 'case_fold_nfkc', 'case_ignorable', 'cased', 'case_changes_casefold', 'case_changes_casefold_nfkc', 'case_changes_casemap', 'case_changes_lower', 'case_changes_upper', 'case_changes_title', 'script', 'script_extensions', 'hangul_syll_type', 'jamo_short_name', 'indic_category_syll', 'indic_category_pos', 'id_start', 'id_start_other', 'id_start_nfkc', 'id_cont', 'id_cont_other', 'id_cont_nfkc', 'patt_syntax', 'patt_white', 'dash', 'quot', 'term_punc', 'term_sentence', 'diacritic', 'extender', 'prepended_concatenation_mark', 'soft_dotted', 'alpha', 'alpha_other', 'math', 'math_other', 'hex_digit', 'hex_digit_ascii', 'default_ignorable', 'default_ignorable_other', 'logical_order_exception', 'white', 'hyphen', 'graph_base', 'graph_ext', 'graph_ext_other', 'graph_link', 'break_graph_cluster', 'break_word', 'break_sentence', 'ideo', 'ideo_unified', 'ideo_desc_seq_bin_op', 'ideo_desc_seq_trin_op', 'ideo_desc_seq_radical', 'deprecated', 'var_sel', 'nonchar']

        return {attr: getattr(self, attr)() for attr in attrs}

        ret = {}
        for attr in attrs:
            m = getattr(self, attr)
            try:
                ret[attr] = m()
            except:
                print('%x'%self._c, attr)
                raise
        return ret


try:
    with lzma.open('ucd.xml.xz', 'rb') as f:
        for (_, el) in et.iterparse(f):
            if '}' in el.tag:
                el.tag = el.tag.split('}', 1)[1]

            if el.tag in ("char", "reserved", "noncharacter", "surrogate"):
                for cp in Codepoint.from_el(el):
                    c = cp.codepoint()
                    if c > 1000:
                        pass#raise StopIteration

                    attrs = cp.all()
                    if c % 8192 == 0:
                        print('%x' % c)

                el.clear()
except StopIteration:
    print('...')
