import re
r2a = lambda raw: [(x, re.sub('_','',x)) for x in raw]

cat_alias = [('Lu', 'UppercaseLetter'), ('Ll', 'LowercaseLetter'), ('Lt', 'TitlecaseLetter'),
             ('Lm', 'ModifierLetter'), ('Lo', 'OtherLetter'), ('Mn', 'NonspacingMark'),
             ('Mc', 'SpacingMark'), ('Me', 'EnclosingMark'), ('Nd', 'DecimalNumber'),
             ('Nl', 'LetterNumber'), ('No', 'OtherNumber'), ('Pc', 'ConnectorPunctuation'),
             ('Pd', 'DashPunctuation'), ('Ps', 'OpenPunctuation'), ('Pe', 'ClosePunctuation'),
             ('Pi', 'InitialPunctuation'), ('Pf', 'FinalPunctuation'), ('Po', 'OtherPunctuation'),
             ('Sm', 'MathSymbol'), ('Sc', 'CurrencySymbol'), ('Sk', 'ModifierSymbol'),
             ('So', 'OtherSymbol'), ('Zs', 'SpaceSeparator'), ('Zl', 'LineSeparator'),
             ('Zp', 'ParagraphSeparator'), ('Cc', 'Control'), ('Cf', 'Format'),
             ('Cs', 'Surrogate'), ('Co', 'PrivateUse'), ('Cn', 'Unassigned')]

bidicl_alias = [('L', 'LeftToRight'), ('R', 'RightToLeft'), ('AL', 'ArabicLetter'),
                ('EN', 'EuropeanNumber'), ('ES', 'EuropeanSeparator'), ('ET', 'EuropeanTerminator'),
                ('AN', 'ArabicNumber'), ('CS', 'CommonSeparator'), ('NSM', 'NonspacingMark'),
                ('BN', 'BoundaryNeutral'), ('B', 'ParagraphSeparator'), ('S', 'SegmentSeparator'),
                ('WS', 'WhiteSpace'), ('ON', 'OtherNeutral'), ('LRE', 'LeftToRightEmbedding'),
                ('LRO', 'LeftToRightOverride'), ('RLE', 'RightToLeftEmbedding'), ('RLO', 'RightToLeftOverride'),
                ('PDF', 'PopDirectionalFormat'), ('LRI', 'LeftToRightIsolate'), ('RLI', 'RightToLeftIsolate'),
                ('FSI', 'FirstStrongIsolate'), ('PDI', 'PopDirectionalIsolate')]

nt_alias = [('Nu', 'Numeric'), ('De', 'Decimal'), ('Di', 'Digit')]

lb_alias = [('AI', 'Ambiguous'), ('AL', 'Alphabetic'), ('B2', 'BreakBoth'), ('BA', 'BreakAfter'),
            ('BB', 'BreakBefore'), ('BK', 'MandatoryBreak'), ('CB', 'ContingentBreak'),
            ('CJ', 'ConditionalJapaneseStarter'), ('CL', 'ClosePunctuation'), ('CM', 'CombiningMark'),
            ('CP', 'CloseParenthesis'), ('CR', 'CarriageReturn'), ('EB', 'EmojiBase'),
            ('EM', 'EmojiModifier'), ('EX', 'Exclamation'), ('GL', 'NonBreakingGlue'),
            ('H2', 'HangulLVSyllable'), ('H3', 'HangulLVTSyllable'), ('HL', 'HebrewLetter'),
            ('HY', 'Hyphen'), ('ID', 'Ideographic'), ('IN', 'Inseparable'), ('IS', 'InfixNumeric'),
            ('JL', 'HangulJamoL'), ('JT', 'HangulJamoT'), ('JV', 'HangulJamoV'), ('LF', 'LineFeed'),
            ('NL', 'NextLine'), ('NS', 'Nonstarter'), ('NU', 'Numeric'), ('OP', 'OpenPunctuation'),
            ('PO', 'PostfixNumeric'), ('PR', 'PrefixNumeric'), ('QU', 'Quotation'),
            ('RI', 'RegionalIndicator'), ('SA', 'ComplexContext'), ('SG', 'Surrogate'),
            ('SP', 'Space'), ('SY', 'BreakSymbols'), ('WJ', 'WordJoiner'), ('XX', 'Unknown'),
            ('ZW', 'ZeroWidthSpace'), ('ZWJ', 'ZeroWidthJoiner')]

ea_alias = [('Na','Narrow'), ('W','Wide'), ('N','Neutral'), ('A','Ambiguous'), ('F','FullWidth'), ('H','HalfWidth')]

jt_alias = [('C', 'JoinCausing'), ('D', 'DualJoining'), ('L', 'LeftJoining'),
            ('R', 'RightJoining'), ('T', 'Transparent'), ('U', 'NonJoining')]

jg_alias = r2a(['African_Feh', 'African_Noon', 'African_Qaf', 'Ain', 'Alaph', 'Alef', 'Beh',
                'Beth', 'Burushaski_Yeh_Barree', 'Dal', 'Dalath_Rish', 'E', 'Farsi_Yeh', 'Fe',
                'Feh', 'Final_Semkath', 'Gaf', 'Gamal', 'Hah', 'He', 'Heh', 'Heh_Goal', 'Heth',
                'Kaf', 'Kaph', 'Khaph', 'Knotted_Heh', 'Lam', 'Lamadh', 'Manichaean_Aleph',
                'Manichaean_Ayin', 'Manichaean_Beth', 'Manichaean_Daleth', 'Manichaean_Dhamedh',
                'Manichaean_Five', 'Manichaean_Gimel', 'Manichaean_Heth', 'Manichaean_Hundred',
                'Manichaean_Kaph', 'Manichaean_Lamedh', 'Manichaean_Mem', 'Manichaean_Nun',
                'Manichaean_One', 'Manichaean_Pe', 'Manichaean_Qoph', 'Manichaean_Resh',
                'Manichaean_Sadhe', 'Manichaean_Samekh', 'Manichaean_Taw', 'Manichaean_Ten',
                'Manichaean_Teth', 'Manichaean_Thamedh', 'Manichaean_Twenty', 'Manichaean_Waw',
                'Manichaean_Yodh', 'Manichaean_Zayin', 'Meem', 'Mim', 'No_Joining_Group', 'Noon',
                'Nun', 'Nya', 'Pe', 'Qaf', 'Qaph', 'Reh', 'Reversed_Pe', 'Rohingya_Yeh', 'Sad',
                'Sadhe', 'Seen', 'Semkath', 'Shin', 'Straight_Waw', 'Swash_Kaf', 'Syriac_Waw',
                'Tah', 'Taw', 'Teh_Marbuta', 'Teh_Marbuta_Goal', 'Teth', 'Waw', 'Yeh',
                'Yeh_Barree', 'Yeh_With_Tail', 'Yudh', 'Yudh_He', 'Zain', 'Zhain'])

sc_alias = [('Adlm', 'Adlam'), ('Aghb', 'CaucasianAlbanian'), ('Ahom', 'Ahom'), ('Arab', 'Arabic'),
            ('Armi', 'ImperialAramaic'), ('Armn', 'Armenian'), ('Avst', 'Avestan'), ('Bali', 'Balinese'),
            ('Bamu', 'Bamum'), ('Bass', 'BassaVah'), ('Batk', 'Batak'), ('Beng', 'Bengali'),
            ('Bhks', 'Bhaiksuki'), ('Bopo', 'Bopomofo'), ('Brah', 'Brahmi'), ('Brai', 'Braille'),
            ('Bugi', 'Buginese'), ('Buhd', 'Buhid'), ('Cakm', 'Chakma'), ('Cans', 'CanadianAboriginal'),
            ('Cari', 'Carian'), ('Cham', 'Cham'), ('Cher', 'Cherokee'), ('Copt', 'Coptic'),
            ('Cprt', 'Cypriot'), ('Cyrl', 'Cyrillic'), ('Deva', 'Devanagari'), ('Dsrt', 'Deseret'),
            ('Dupl', 'Duployan'), ('Egyp', 'EgyptianHieroglyphs'), ('Elba', 'Elbasan'), ('Ethi', 'Ethiopic'),
            ('Geor', 'Georgian'), ('Glag', 'Glagolitic'), ('Goth', 'Gothic'), ('Gran', 'Grantha'),
            ('Grek', 'Greek'), ('Gujr', 'Gujarati'), ('Guru', 'Gurmukhi'), ('Hang', 'Hangul'),
            ('Hani', 'Han'), ('Hano', 'Hanunoo'), ('Hatr', 'Hatran'), ('Hebr', 'Hebrew'),
            ('Hira', 'Hiragana'), ('Hluw', 'AnatolianHieroglyphs'), ('Hmng', 'PahawhHmong'),
            ('Hrkt', 'KatakanaOrHiragana'), ('Hung', 'OldHungarian'), ('Ital', 'OldItalic'),
            ('Java', 'Javanese'), ('Kali', 'KayahLi'), ('Kana', 'Katakana'), ('Khar', 'Kharoshthi'),
            ('Khmr', 'Khmer'), ('Khoj', 'Khojki'), ('Knda', 'Kannada'), ('Kthi', 'Kaithi'),
            ('Lana', 'TaiTham'), ('Laoo', 'Lao'), ('Latn', 'Latin'), ('Lepc', 'Lepcha'), ('Limb', 'Limbu'),
            ('Lina', 'LinearA'), ('Linb', 'LinearB'), ('Lisu', 'Lisu'), ('Lyci', 'Lycian'),
            ('Lydi', 'Lydian'), ('Mahj', 'Mahajani'), ('Mand', 'Mandaic'), ('Mani', 'Manichaean'),
            ('Marc', 'Marchen'), ('Mend', 'MendeKikakui'), ('Merc', 'MeroiticCursive'),
            ('Mero', 'MeroiticHieroglyphs'), ('Mlym', 'Malayalam'), ('Modi', 'Modi'), ('Mong', 'Mongolian'),
            ('Mroo', 'Mro'), ('Mtei', 'MeeteiMayek'), ('Mult', 'Multani'), ('Mymr', 'Myanmar'),
            ('Narb', 'OldNorthArabian'), ('Nbat', 'Nabataean'), ('Newa', 'Newa'), ('Nkoo', 'Nko'),
            ('Ogam', 'Ogham'), ('Olck', 'OlChiki'), ('Orkh', 'OldTurkic'), ('Orya', 'Oriya'),
            ('Osge', 'Osage'), ('Osma', 'Osmanya'), ('Palm', 'Palmyrene'), ('Pauc', 'PauCinHau'),
            ('Perm', 'OldPermic'), ('Phag', 'PhagsPa'), ('Phli', 'InscriptionalPahlavi'),
            ('Phlp', 'PsalterPahlavi'), ('Phnx', 'Phoenician'), ('Plrd', 'Miao'),
            ('Prti', 'InscriptionalParthian'), ('Rjng', 'Rejang'), ('Runr', 'Runic'), ('Samr', 'Samaritan'),
            ('Sarb', 'OldSouthArabian'), ('Saur', 'Saurashtra'), ('Sgnw', 'SignWriting'), ('Shaw', 'Shavian'),
            ('Shrd', 'Sharada'), ('Sidd', 'Siddham'), ('Sind', 'Khudawadi'), ('Sinh', 'Sinhala'),
            ('Sora', 'SoraSompeng'), ('Sund', 'Sundanese'), ('Sylo', 'SylotiNagri'), ('Syrc', 'Syriac'),
            ('Tagb', 'Tagbanwa'), ('Takr', 'Takri'), ('Tale', 'TaiLe'), ('Talu', 'NewTaiLue'),
            ('Taml', 'Tamil'), ('Tang', 'Tangut'), ('Tavt', 'TaiViet'), ('Telu', 'Telugu'),
            ('Tfng', 'Tifinagh'), ('Tglg', 'Tagalog'), ('Thaa', 'Thaana'), ('Thai', 'Thai'),
            ('Tibt', 'Tibetan'), ('Tirh', 'Tirhuta'), ('Ugar', 'Ugaritic'), ('Vaii', 'Vai'),
            ('Wara', 'WarangCiti'), ('Xpeo', 'OldPersian'), ('Xsux', 'Cuneiform'), ('Yiii', 'Yi'),
            ('Zinh', 'Inherited'), ('Zyyy', 'Common')]

hst_alias = [('L', 'LeadingJamo'), ('V', 'VowelJamo'), ('T', 'TrailingJamo'),
             ('LV', 'LVSyllable'), ('LVT', 'LVTSyllable')]

insc_alias = r2a(['Consonant_Dead', 'Tone_Mark', 'Tone_Letter', 'Vowel_Dependent', 'Number_Joiner', 'Virama',
                  'Consonant_Head_Letter', 'Number', 'Consonant_Medial', 'Modifying_Letter',
                  'Consonant_Succeeding_Repha', 'Consonant_Final', 'Avagraha', 'Vowel', 'Vowel_Independent',
                  'Register_Shifter', 'Consonant_Killer', 'Other', 'Consonant_Subjoined', 'Joiner', 'Nukta',
                  'Gemination_Mark', 'Invisible_Stacker', 'Consonant_With_Stacker', 'Consonant_Placeholder',
                  'Consonant_Preceding_Repha', 'Cantillation_Mark', 'Pure_Killer', 'Non_Joiner',
                  'Syllable_Modifier', 'Brahmi_Joining_Number', 'Bindu', 'Visarga', 'Consonant', 'Consonant_Prefixed'])

inpc_alias = r2a(['Left', 'Right', 'Top_And_Bottom_And_Right', 'Top_And_Left', 'Top_And_Right', 'Top',
                  'Bottom_And_Right', 'Overstruck', 'Left_And_Right', 'Top_And_Left_And_Right',
                  'Top_And_Bottom', 'Bottom', 'Visual_Order_Left'])
