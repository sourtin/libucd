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

import re
jg_raw = ['African_Feh', 'African_Noon', 'African_Qaf', 'Ain', 'Alaph', 'Alef', 'Beh',
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
          'Yeh_Barree', 'Yeh_With_Tail', 'Yudh', 'Yudh_He', 'Zain', 'Zhain']
jg_alias = [(x, re.sub('_','',x)) for x in jg_raw]
