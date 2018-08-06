// Each ligature is of the format:
//
//   ("<key>", <replacement>)
//
// Where <key> is used in the configuration and <replacement> is of the format:
//
//   ("<match>", ("<isolated>", "<initial>", "<medial>", "<final>"))
//
// Where <match> is the string to replace, and <isolated> is the replacement in
// case <match> was in isolated form, <initial> is the replacement in case
// <match> was in initial form, <medial> is the replacement in case <match> was
// in medial form, and <final> is the replacement in case <match> was in final
// form. If no replacement is specified for a form, then no replacement of
// <match> will occur.

// Order here is important, it should be:
//   1. Sentences
//   2. Words
//   3. Letters
// This way we make sure we replace the longest ligatures first

pub const LIGATURES: [(&str, (&str, [&str; 4])); 306] = [
    // Sentences
    (
        "ARABIC LIGATURE BISMILLAH AR-RAHMAN AR-RAHEEM",
        (
            "\u{0628}\u{0633}\u{0645}\u{0020}
        \u{0627}\u{0644}\u{0644}\u{0647}\u{0020}
        \u{0627}\u{0644}\u{0631}\u{062D}\u{0645}\u{0646}\u{0020}
        \u{0627}\u{0644}\u{0631}\u{062D}\u{064A}\u{0645}",
            ["\u{FDFD}", "", "", ""],
        ),
    ),
    (
        "ARABIC LIGATURE JALLAJALALOUHOU",
        (
            "\u{062C}\u{0644}\u{0020}\u{062C}\u{0644}\u{0627}\u{0644}\u{0647}",
            ["\u{FDFB}", "", "", ""],
        ),
    ),
    (
        "ARABIC LIGATURE SALLALLAHOU ALAYHE WASALLAM",
        (
            "\u{0635}\u{0644}\u{0649}\u{0020}
        \u{0627}\u{0644}\u{0644}\u{0647}\u{0020}
        \u{0639}\u{0644}\u{064A}\u{0647}\u{0020}
        \u{0648}\u{0633}\u{0644}\u{0645}",
            ["\u{FDFA}", "", "", ""],
        ),
    ),
    // Words
    (
        "ARABIC LIGATURE ALLAH",
        ("\u{0627}\u{0644}\u{0644}\u{0647}", ["\u{FDF2}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE AKBAR",
        ("\u{0623}\u{0643}\u{0628}\u{0631}", ["\u{FDF3}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE ALAYHE",
        ("\u{0639}\u{0644}\u{064A}\u{0647}", ["\u{FDF7}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE MOHAMMAD",
        ("\u{0645}\u{062D}\u{0645}\u{062F}", ["\u{FDF4}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE RASOUL",
        ("\u{0631}\u{0633}\u{0648}\u{0644}", ["\u{FDF6}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE SALAM",
        ("\u{0635}\u{0644}\u{0639}\u{0645}", ["\u{FDF5}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE SALLA",
        ("\u{0635}\u{0644}\u{0649}", ["\u{FDF9}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE WASALLAM",
        ("\u{0648}\u{0633}\u{0644}\u{0645}", ["\u{FDF8}", "", "", ""]),
    ),
    (
        "RIAL SIGN",
        (
            "\u{0631}[\u{06CC}\u{064A}]\u{0627}\u{0644}",
            ["\u{FDFC}", "", "", ""],
        ),
    ),
    // Letters
    (
        "ARABIC LIGATURE AIN WITH ALEF MAKSURA",
        ("\u{0639}\u{0649}", ["\u{FCF7}", "", "", "\u{FD13}"]),
    ),
    (
        "ARABIC LIGATURE AIN WITH JEEM",
        ("\u{0639}\u{062C}", ["\u{FC29}", "\u{FCBA}", "", ""]),
    ),
    (
        "ARABIC LIGATURE AIN WITH JEEM WITH MEEM",
        ("\u{0639}\u{062C}\u{0645}", ["", "\u{FDC4}", "", "\u{FD75}"]),
    ),
    (
        "ARABIC LIGATURE AIN WITH MEEM",
        ("\u{0639}\u{0645}", ["\u{FC2A}", "\u{FCBB}", "", ""]),
    ),
    (
        "ARABIC LIGATURE AIN WITH MEEM WITH ALEF MAKSURA",
        ("\u{0639}\u{0645}\u{0649}", ["", "", "", "\u{FD78}"]),
    ),
    (
        "ARABIC LIGATURE AIN WITH MEEM WITH MEEM",
        ("\u{0639}\u{0645}\u{0645}", ["", "\u{FD77}", "", "\u{FD76}"]),
    ),
    (
        "ARABIC LIGATURE AIN WITH MEEM WITH YEH",
        ("\u{0639}\u{0645}\u{064A}", ["", "", "", "\u{FDB6}"]),
    ),
    (
        "ARABIC LIGATURE AIN WITH YEH",
        ("\u{0639}\u{064A}", ["\u{FCF8}", "", "", "\u{FD14}"]),
    ),
    (
        "ARABIC LIGATURE ALEF MAKSURA WITH SUPERSCRIPT ALEF",
        ("\u{0649}\u{0670}", ["\u{FC5D}", "", "", "\u{FC90}"]),
    ),
    (
        "ARABIC LIGATURE ALEF WITH FATHATAN",
        ("\u{0627}\u{064B}", ["\u{FD3D}", "", "", "\u{FD3C}"]),
    ),
    (
        "ARABIC LIGATURE BEH WITH ALEF MAKSURA",
        ("\u{0628}\u{0649}", ["\u{FC09}", "", "", "\u{FC6E}"]),
    ),
    (
        "ARABIC LIGATURE BEH WITH HAH",
        ("\u{0628}\u{062D}", ["\u{FC06}", "\u{FC9D}", "", ""]),
    ),
    (
        "ARABIC LIGATURE BEH WITH HAH WITH YEH",
        ("\u{0628}\u{062D}\u{064A}", ["", "", "", "\u{FDC2}"]),
    ),
    (
        "ARABIC LIGATURE BEH WITH HEH",
        ("\u{0628}\u{0647}", ["", "\u{FCA0}", "\u{FCE2}", ""]),
    ),
    (
        "ARABIC LIGATURE BEH WITH JEEM",
        ("\u{0628}\u{062C}", ["\u{FC05}", "\u{FC9C}", "", ""]),
    ),
    (
        "ARABIC LIGATURE BEH WITH KHAH",
        ("\u{0628}\u{062E}", ["\u{FC07}", "\u{FC9E}", "", ""]),
    ),
    (
        "ARABIC LIGATURE BEH WITH KHAH WITH YEH",
        ("\u{0628}\u{062E}\u{064A}", ["", "", "", "\u{FD9E}"]),
    ),
    (
        "ARABIC LIGATURE BEH WITH MEEM",
        (
            "\u{0628}\u{0645}",
            ["\u{FC08}", "\u{FC9F}", "\u{FCE1}", "\u{FC6C}"],
        ),
    ),
    (
        "ARABIC LIGATURE BEH WITH NOON",
        ("\u{0628}\u{0646}", ["", "", "", "\u{FC6D}"]),
    ),
    (
        "ARABIC LIGATURE BEH WITH REH",
        ("\u{0628}\u{0631}", ["", "", "", "\u{FC6A}"]),
    ),
    (
        "ARABIC LIGATURE AIN WITH ALEF MAKSURA",
        ("\u{0639}\u{0649}", ["\u{FCF7}", "", "", "\u{FD13}"]),
    ),
    (
        "ARABIC LIGATURE AIN WITH JEEM",
        ("\u{0639}\u{062C}", ["\u{FC29}", "\u{FCBA}", "", ""]),
    ),
    (
        "ARABIC LIGATURE AIN WITH JEEM WITH MEEM",
        ("\u{0639}\u{062C}\u{0645}", ["", "\u{FDC4}", "", "\u{FD75}"]),
    ),
    (
        "ARABIC LIGATURE AIN WITH MEEM",
        ("\u{0639}\u{0645}", ["\u{FC2A}", "\u{FCBB}", "", ""]),
    ),
    (
        "ARABIC LIGATURE AIN WITH MEEM WITH ALEF MAKSURA",
        ("\u{0639}\u{0645}\u{0649}", ["", "", "", "\u{FD78}"]),
    ),
    (
        "ARABIC LIGATURE AIN WITH MEEM WITH MEEM",
        ("\u{0639}\u{0645}\u{0645}", ["", "\u{FD77}", "", "\u{FD76}"]),
    ),
    (
        "ARABIC LIGATURE AIN WITH MEEM WITH YEH",
        ("\u{0639}\u{0645}\u{064A}", ["", "", "", "\u{FDB6}"]),
    ),
    (
        "ARABIC LIGATURE AIN WITH YEH",
        ("\u{0639}\u{064A}", ["\u{FCF8}", "", "", "\u{FD14}"]),
    ),
    (
        "ARABIC LIGATURE ALEF MAKSURA WITH SUPERSCRIPT ALEF",
        ("\u{0649}\u{0670}", ["\u{FC5D}", "", "", "\u{FC90}"]),
    ),
    (
        "ARABIC LIGATURE ALEF WITH FATHATAN",
        ("\u{0627}\u{064B}", ["\u{FD3D}", "", "", "\u{FD3C}"]),
    ),
    (
        "ARABIC LIGATURE BEH WITH ALEF MAKSURA",
        ("\u{0628}\u{0649}", ["\u{FC09}", "", "", "\u{FC6E}"]),
    ),
    (
        "ARABIC LIGATURE BEH WITH HAH",
        ("\u{0628}\u{062D}", ["\u{FC06}", "\u{FC9D}", "", ""]),
    ),
    (
        "ARABIC LIGATURE BEH WITH HAH WITH YEH",
        ("\u{0628}\u{062D}\u{064A}", ["", "", "", "\u{FDC2}"]),
    ),
    (
        "ARABIC LIGATURE BEH WITH HEH",
        ("\u{0628}\u{0647}", ["", "\u{FCA0}", "\u{FCE2}", ""]),
    ),
    (
        "ARABIC LIGATURE BEH WITH JEEM",
        ("\u{0628}\u{062C}", ["\u{FC05}", "\u{FC9C}", "", ""]),
    ),
    (
        "ARABIC LIGATURE BEH WITH KHAH",
        ("\u{0628}\u{062E}", ["\u{FC07}", "\u{FC9E}", "", ""]),
    ),
    (
        "ARABIC LIGATURE BEH WITH KHAH WITH YEH",
        ("\u{0628}\u{062E}\u{064A}", ["", "", "", "\u{FD9E}"]),
    ),
    (
        "ARABIC LIGATURE BEH WITH MEEM",
        (
            "\u{0628}\u{0645}",
            ["\u{FC08}", "\u{FC9F}", "\u{FCE1}", "\u{FC6C}"],
        ),
    ),
    (
        "ARABIC LIGATURE BEH WITH NOON",
        ("\u{0628}\u{0646}", ["", "", "", "\u{FC6D}"]),
    ),
    (
        "ARABIC LIGATURE BEH WITH REH",
        ("\u{0628}\u{0631}", ["", "", "", "\u{FC6A}"]),
    ),
    (
        "ARABIC LIGATURE BEH WITH YEH",
        ("\u{0628}\u{064A}", ["\u{FC0A}", "", "", "\u{FC6F}"]),
    ),
    (
        "ARABIC LIGATURE BEH WITH ZAIN",
        ("\u{0628}\u{0632}", ["", "", "", "\u{FC6B}"]),
    ),
    (
        "ARABIC LIGATURE DAD WITH ALEF MAKSURA",
        ("\u{0636}\u{0649}", ["\u{FD07}", "", "", "\u{FD23}"]),
    ),
    (
        "ARABIC LIGATURE DAD WITH HAH",
        ("\u{0636}\u{062D}", ["\u{FC23}", "\u{FCB5}", "", ""]),
    ),
    (
        "ARABIC LIGATURE DAD WITH HAH WITH ALEF MAKSURA",
        ("\u{0636}\u{062D}\u{0649}", ["", "", "", "\u{FD6E}"]),
    ),
    (
        "ARABIC LIGATURE DAD WITH HAH WITH YEH",
        ("\u{0636}\u{062D}\u{064A}", ["", "", "", "\u{FDAB}"]),
    ),
    (
        "ARABIC LIGATURE DAD WITH JEEM",
        ("\u{0636}\u{062C}", ["\u{FC22}", "\u{FCB4}", "", ""]),
    ),
    (
        "ARABIC LIGATURE DAD WITH KHAH",
        ("\u{0636}\u{062E}", ["\u{FC24}", "\u{FCB6}", "", ""]),
    ),
    (
        "ARABIC LIGATURE DAD WITH KHAH WITH MEEM",
        ("\u{0636}\u{062E}\u{0645}", ["", "\u{FD70}", "", "\u{FD6F}"]),
    ),
    (
        "ARABIC LIGATURE DAD WITH MEEM",
        ("\u{0636}\u{0645}", ["\u{FC25}", "\u{FCB7}", "", ""]),
    ),
    (
        "ARABIC LIGATURE DAD WITH REH",
        ("\u{0636}\u{0631}", ["\u{FD10}", "", "", "\u{FD2C}"]),
    ),
    (
        "ARABIC LIGATURE DAD WITH YEH",
        ("\u{0636}\u{064A}", ["\u{FD08}", "", "", "\u{FD24}"]),
    ),
    (
        "ARABIC LIGATURE FEH WITH ALEF MAKSURA",
        ("\u{0641}\u{0649}", ["\u{FC31}", "", "", "\u{FC7C}"]),
    ),
    (
        "ARABIC LIGATURE FEH WITH HAH",
        ("\u{0641}\u{062D}", ["\u{FC2E}", "\u{FCBF}", "", ""]),
    ),
    (
        "ARABIC LIGATURE FEH WITH JEEM",
        ("\u{0641}\u{062C}", ["\u{FC2D}", "\u{FCBE}", "", ""]),
    ),
    (
        "ARABIC LIGATURE FEH WITH KHAH",
        ("\u{0641}\u{062E}", ["\u{FC2F}", "\u{FCC0}", "", ""]),
    ),
    (
        "ARABIC LIGATURE FEH WITH KHAH WITH MEEM",
        ("\u{0641}\u{062E}\u{0645}", ["", "\u{FD7D}", "", "\u{FD7C}"]),
    ),
    (
        "ARABIC LIGATURE FEH WITH MEEM",
        ("\u{0641}\u{0645}", ["\u{FC30}", "\u{FCC1}", "", ""]),
    ),
    (
        "ARABIC LIGATURE FEH WITH MEEM WITH YEH",
        ("\u{0641}\u{0645}\u{064A}", ["", "", "", "\u{FDC1}"]),
    ),
    (
        "ARABIC LIGATURE FEH WITH YEH",
        ("\u{0641}\u{064A}", ["\u{FC32}", "", "", "\u{FC7D}"]),
    ),
    (
        "ARABIC LIGATURE GHAIN WITH ALEF MAKSURA",
        ("\u{063A}\u{0649}", ["\u{FCF9}", "", "", "\u{FD15}"]),
    ),
    (
        "ARABIC LIGATURE GHAIN WITH JEEM",
        ("\u{063A}\u{062C}", ["\u{FC2B}", "\u{FCBC}", "", ""]),
    ),
    (
        "ARABIC LIGATURE GHAIN WITH MEEM",
        ("\u{063A}\u{0645}", ["\u{FC2C}", "\u{FCBD}", "", ""]),
    ),
    (
        "ARABIC LIGATURE GHAIN WITH MEEM WITH ALEF MAKSURA",
        ("\u{063A}\u{0645}\u{0649}", ["", "", "", "\u{FD7B}"]),
    ),
    (
        "ARABIC LIGATURE GHAIN WITH MEEM WITH MEEM",
        ("\u{063A}\u{0645}\u{0645}", ["", "", "", "\u{FD79}"]),
    ),
    (
        "ARABIC LIGATURE GHAIN WITH MEEM WITH YEH",
        ("\u{063A}\u{0645}\u{064A}", ["", "", "", "\u{FD7A}"]),
    ),
    (
        "ARABIC LIGATURE GHAIN WITH YEH",
        ("\u{063A}\u{064A}", ["\u{FCFA}", "", "", "\u{FD16}"]),
    ),
    (
        "ARABIC LIGATURE HAH WITH ALEF MAKSURA",
        ("\u{062D}\u{0649}", ["\u{FCFF}", "", "", "\u{FD1B}"]),
    ),
    (
        "ARABIC LIGATURE HAH WITH JEEM",
        ("\u{062D}\u{062C}", ["\u{FC17}", "\u{FCA9}", "", ""]),
    ),
    (
        "ARABIC LIGATURE HAH WITH JEEM WITH YEH",
        ("\u{062D}\u{062C}\u{064A}", ["", "", "", "\u{FDBF}"]),
    ),
    (
        "ARABIC LIGATURE HAH WITH MEEM",
        ("\u{062D}\u{0645}", ["\u{FC18}", "\u{FCAA}", "", ""]),
    ),
    (
        "ARABIC LIGATURE HAH WITH MEEM WITH ALEF MAKSURA",
        ("\u{062D}\u{0645}\u{0649}", ["", "", "", "\u{FD5B}"]),
    ),
    (
        "ARABIC LIGATURE HAH WITH MEEM WITH YEH",
        ("\u{062D}\u{0645}\u{064A}", ["", "", "", "\u{FD5A}"]),
    ),
    (
        "ARABIC LIGATURE HAH WITH YEH",
        ("\u{062D}\u{064A}", ["\u{FD00}", "", "", "\u{FD1C}"]),
    ),
    (
        "ARABIC LIGATURE HEH WITH ALEF MAKSURA",
        ("\u{0647}\u{0649}", ["\u{FC53}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE HEH WITH JEEM",
        ("\u{0647}\u{062C}", ["\u{FC51}", "\u{FCD7}", "", ""]),
    ),
    (
        "ARABIC LIGATURE HEH WITH MEEM",
        ("\u{0647}\u{0645}", ["\u{FC52}", "\u{FCD8}", "", ""]),
    ),
    (
        "ARABIC LIGATURE HEH WITH MEEM WITH JEEM",
        ("\u{0647}\u{0645}\u{062C}", ["", "\u{FD93}", "", ""]),
    ),
    (
        "ARABIC LIGATURE HEH WITH MEEM WITH MEEM",
        ("\u{0647}\u{0645}\u{0645}", ["", "\u{FD94}", "", ""]),
    ),
    (
        "ARABIC LIGATURE HEH WITH SUPERSCRIPT ALEF",
        ("\u{0647}\u{0670}", ["", "\u{FCD9}", "", ""]),
    ),
    (
        "ARABIC LIGATURE HEH WITH YEH",
        ("\u{0647}\u{064A}", ["\u{FC54}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE JEEM WITH ALEF MAKSURA",
        ("\u{062C}\u{0649}", ["\u{FD01}", "", "", "\u{FD1D}"]),
    ),
    (
        "ARABIC LIGATURE JEEM WITH HAH",
        ("\u{062C}\u{062D}", ["\u{FC15}", "\u{FCA7}", "", ""]),
    ),
    (
        "ARABIC LIGATURE JEEM WITH HAH WITH ALEF MAKSURA",
        ("\u{062C}\u{062D}\u{0649}", ["", "", "", "\u{FDA6}"]),
    ),
    (
        "ARABIC LIGATURE JEEM WITH HAH WITH YEH",
        ("\u{062C}\u{062D}\u{064A}", ["", "", "", "\u{FDBE}"]),
    ),
    (
        "ARABIC LIGATURE JEEM WITH MEEM",
        ("\u{062C}\u{0645}", ["\u{FC16}", "\u{FCA8}", "", ""]),
    ),
    (
        "ARABIC LIGATURE JEEM WITH MEEM WITH ALEF MAKSURA",
        ("\u{062C}\u{0645}\u{0649}", ["", "", "", "\u{FDA7}"]),
    ),
    (
        "ARABIC LIGATURE JEEM WITH MEEM WITH HAH",
        ("\u{062C}\u{0645}\u{062D}", ["", "\u{FD59}", "", "\u{FD58}"]),
    ),
    (
        "ARABIC LIGATURE JEEM WITH MEEM WITH YEH",
        ("\u{062C}\u{0645}\u{064A}", ["", "", "", "\u{FDA5}"]),
    ),
    (
        "ARABIC LIGATURE JEEM WITH YEH",
        ("\u{062C}\u{064A}", ["\u{FD02}", "", "", "\u{FD1E}"]),
    ),
    (
        "ARABIC LIGATURE KAF WITH ALEF",
        ("\u{0643}\u{0627}", ["\u{FC37}", "", "", "\u{FC80}"]),
    ),
    (
        "ARABIC LIGATURE KAF WITH ALEF MAKSURA",
        ("\u{0643}\u{0649}", ["\u{FC3D}", "", "", "\u{FC83}"]),
    ),
    (
        "ARABIC LIGATURE KAF WITH HAH",
        ("\u{0643}\u{062D}", ["\u{FC39}", "\u{FCC5}", "", ""]),
    ),
    (
        "ARABIC LIGATURE KAF WITH JEEM",
        ("\u{0643}\u{062C}", ["\u{FC38}", "\u{FCC4}", "", ""]),
    ),
    (
        "ARABIC LIGATURE KAF WITH KHAH",
        ("\u{0643}\u{062E}", ["\u{FC3A}", "\u{FCC6}", "", ""]),
    ),
    (
        "ARABIC LIGATURE KAF WITH LAM",
        (
            "\u{0643}\u{0644}",
            ["\u{FC3B}", "\u{FCC7}", "\u{FCEB}", "\u{FC81}"],
        ),
    ),
    (
        "ARABIC LIGATURE KAF WITH MEEM",
        (
            "\u{0643}\u{0645}",
            ["\u{FC3C}", "\u{FCC8}", "\u{FCEC}", "\u{FC82}"],
        ),
    ),
    (
        "ARABIC LIGATURE KAF WITH MEEM WITH MEEM",
        ("\u{0643}\u{0645}\u{0645}", ["", "\u{FDC3}", "", "\u{FDBB}"]),
    ),
    (
        "ARABIC LIGATURE KAF WITH MEEM WITH YEH",
        ("\u{0643}\u{0645}\u{064A}", ["", "", "", "\u{FDB7}"]),
    ),
    (
        "ARABIC LIGATURE KAF WITH YEH",
        ("\u{0643}\u{064A}", ["\u{FC3E}", "", "", "\u{FC84}"]),
    ),
    (
        "ARABIC LIGATURE KHAH WITH ALEF MAKSURA",
        ("\u{062E}\u{0649}", ["\u{FD03}", "", "", "\u{FD1F}"]),
    ),
    (
        "ARABIC LIGATURE KHAH WITH HAH",
        ("\u{062E}\u{062D}", ["\u{FC1A}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE KHAH WITH JEEM",
        ("\u{062E}\u{062C}", ["\u{FC19}", "\u{FCAB}", "", ""]),
    ),
    (
        "ARABIC LIGATURE KHAH WITH MEEM",
        ("\u{062E}\u{0645}", ["\u{FC1B}", "\u{FCAC}", "", ""]),
    ),
    (
        "ARABIC LIGATURE KHAH WITH YEH",
        ("\u{062E}\u{064A}", ["\u{FD04}", "", "", "\u{FD20}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH ALEF",
        ("\u{0644}\u{0627}", ["\u{FEFB}", "", "", "\u{FEFC}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH ALEF MAKSURA",
        ("\u{0644}\u{0649}", ["\u{FC43}", "", "", "\u{FC86}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH ALEF WITH HAMZA ABOVE",
        ("\u{0644}\u{0623}", ["\u{FEF7}", "", "", "\u{FEF8}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH ALEF WITH HAMZA BELOW",
        ("\u{0644}\u{0625}", ["\u{FEF9}", "", "", "\u{FEFA}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH ALEF WITH MADDA ABOVE",
        ("\u{0644}\u{0622}", ["\u{FEF5}", "", "", "\u{FEF6}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH HAH",
        ("\u{0644}\u{062D}", ["\u{FC40}", "\u{FCCA}", "", ""]),
    ),
    (
        "ARABIC LIGATURE LAM WITH HAH WITH ALEF MAKSURA",
        ("\u{0644}\u{062D}\u{0649}", ["", "", "", "\u{FD82}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH HAH WITH MEEM",
        ("\u{0644}\u{062D}\u{0645}", ["", "\u{FDB5}", "", "\u{FD80}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH HAH WITH YEH",
        ("\u{0644}\u{062D}\u{064A}", ["", "", "", "\u{FD81}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH HEH",
        ("\u{0644}\u{0647}", ["", "\u{FCCD}", "", ""]),
    ),
    (
        "ARABIC LIGATURE LAM WITH JEEM",
        ("\u{0644}\u{062C}", ["\u{FC3F}", "\u{FCC9}", "", ""]),
    ),
    (
        "ARABIC LIGATURE LAM WITH JEEM WITH JEEM",
        ("\u{0644}\u{062C}\u{062C}", ["", "\u{FD83}", "", "\u{FD84}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH JEEM WITH MEEM",
        ("\u{0644}\u{062C}\u{0645}", ["", "\u{FDBA}", "", "\u{FDBC}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH JEEM WITH YEH",
        ("\u{0644}\u{062C}\u{064A}", ["", "", "", "\u{FDAC}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH KHAH",
        ("\u{0644}\u{062E}", ["\u{FC41}", "\u{FCCB}", "", ""]),
    ),
    (
        "ARABIC LIGATURE LAM WITH KHAH WITH MEEM",
        ("\u{0644}\u{062E}\u{0645}", ["", "\u{FD86}", "", "\u{FD85}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH MEEM",
        (
            "\u{0644}\u{0645}",
            ["\u{FC42}", "\u{FCCC}", "\u{FCED}", "\u{FC85}"],
        ),
    ),
    (
        "ARABIC LIGATURE LAM WITH MEEM WITH HAH",
        ("\u{0644}\u{0645}\u{062D}", ["", "\u{FD88}", "", "\u{FD87}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH MEEM WITH YEH",
        ("\u{0644}\u{0645}\u{064A}", ["", "", "", "\u{FDAD}"]),
    ),
    (
        "ARABIC LIGATURE LAM WITH YEH",
        ("\u{0644}\u{064A}", ["\u{FC44}", "", "", "\u{FC87}"]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH ALEF",
        ("\u{0645}\u{0627}", ["", "", "", "\u{FC88}"]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH ALEF MAKSURA",
        ("\u{0645}\u{0649}", ["\u{FC49}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH HAH",
        ("\u{0645}\u{062D}", ["\u{FC46}", "\u{FCCF}", "", ""]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH HAH WITH JEEM",
        ("\u{0645}\u{062D}\u{062C}", ["", "\u{FD89}", "", ""]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH HAH WITH MEEM",
        ("\u{0645}\u{062D}\u{0645}", ["", "\u{FD8A}", "", ""]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH HAH WITH YEH",
        ("\u{0645}\u{062D}\u{064A}", ["", "", "", "\u{FD8B}"]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH JEEM",
        ("\u{0645}\u{062C}", ["\u{FC45}", "\u{FCCE}", "", ""]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH JEEM WITH HAH",
        ("\u{0645}\u{062C}\u{062D}", ["", "\u{FD8C}", "", ""]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH JEEM WITH KHAH",
        ("\u{0645}\u{062C}\u{062E}", ["", "\u{FD92}", "", ""]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH JEEM WITH MEEM",
        ("\u{0645}\u{062C}\u{0645}", ["", "\u{FD8D}", "", ""]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH JEEM WITH YEH",
        ("\u{0645}\u{062C}\u{064A}", ["", "", "", "\u{FDC0}"]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH KHAH",
        ("\u{0645}\u{062E}", ["\u{FC47}", "\u{FCD0}", "", ""]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH KHAH WITH JEEM",
        ("\u{0645}\u{062E}\u{062C}", ["", "\u{FD8E}", "", ""]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH KHAH WITH MEEM",
        ("\u{0645}\u{062E}\u{0645}", ["", "\u{FD8F}", "", ""]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH KHAH WITH YEH",
        ("\u{0645}\u{062E}\u{064A}", ["", "", "", "\u{FDB9}"]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH MEEM",
        ("\u{0645}\u{0645}", ["\u{FC48}", "\u{FCD1}", "", "\u{FC89}"]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH MEEM WITH YEH",
        ("\u{0645}\u{0645}\u{064A}", ["", "", "", "\u{FDB1}"]),
    ),
    (
        "ARABIC LIGATURE MEEM WITH YEH",
        ("\u{0645}\u{064A}", ["\u{FC4A}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE NOON WITH ALEF MAKSURA",
        ("\u{0646}\u{0649}", ["\u{FC4F}", "", "", "\u{FC8E}"]),
    ),
    (
        "ARABIC LIGATURE NOON WITH HAH",
        ("\u{0646}\u{062D}", ["\u{FC4C}", "\u{FCD3}", "", ""]),
    ),
    (
        "ARABIC LIGATURE NOON WITH HAH WITH ALEF MAKSURA",
        ("\u{0646}\u{062D}\u{0649}", ["", "", "", "\u{FD96}"]),
    ),
    (
        "ARABIC LIGATURE NOON WITH HAH WITH MEEM",
        ("\u{0646}\u{062D}\u{0645}", ["", "\u{FD95}", "", ""]),
    ),
    (
        "ARABIC LIGATURE NOON WITH HAH WITH YEH",
        ("\u{0646}\u{062D}\u{064A}", ["", "", "", "\u{FDB3}"]),
    ),
    (
        "ARABIC LIGATURE NOON WITH HEH",
        ("\u{0646}\u{0647}", ["", "\u{FCD6}", "\u{FCEF}", ""]),
    ),
    (
        "ARABIC LIGATURE NOON WITH JEEM",
        ("\u{0646}\u{062C}", ["\u{FC4B}", "\u{FCD2}", "", ""]),
    ),
    (
        "ARABIC LIGATURE NOON WITH JEEM WITH ALEF MAKSURA",
        ("\u{0646}\u{062C}\u{0649}", ["", "", "", "\u{FD99}"]),
    ),
    (
        "ARABIC LIGATURE NOON WITH JEEM WITH HAH",
        ("\u{0646}\u{062C}\u{062D}", ["", "\u{FDB8}", "", "\u{FDBD}"]),
    ),
    (
        "ARABIC LIGATURE NOON WITH JEEM WITH MEEM",
        ("\u{0646}\u{062C}\u{0645}", ["", "\u{FD98}", "", "\u{FD97}"]),
    ),
    (
        "ARABIC LIGATURE NOON WITH JEEM WITH YEH",
        ("\u{0646}\u{062C}\u{064A}", ["", "", "", "\u{FDC7}"]),
    ),
    (
        "ARABIC LIGATURE NOON WITH KHAH",
        ("\u{0646}\u{062E}", ["\u{FC4D}", "\u{FCD4}", "", ""]),
    ),
    (
        "ARABIC LIGATURE NOON WITH MEEM",
        (
            "\u{0646}\u{0645}",
            ["\u{FC4E}", "\u{FCD5}", "\u{FCEE}", "\u{FC8C}"],
        ),
    ),
    (
        "ARABIC LIGATURE NOON WITH MEEM WITH ALEF MAKSURA",
        ("\u{0646}\u{0645}\u{0649}", ["", "", "", "\u{FD9B}"]),
    ),
    (
        "ARABIC LIGATURE NOON WITH MEEM WITH YEH",
        ("\u{0646}\u{0645}\u{064A}", ["", "", "", "\u{FD9A}"]),
    ),
    (
        "ARABIC LIGATURE NOON WITH NOON",
        ("\u{0646}\u{0646}", ["", "", "", "\u{FC8D}"]),
    ),
    (
        "ARABIC LIGATURE NOON WITH REH",
        ("\u{0646}\u{0631}", ["", "", "", "\u{FC8A}"]),
    ),
    (
        "ARABIC LIGATURE NOON WITH YEH",
        ("\u{0646}\u{064A}", ["\u{FC50}", "", "", "\u{FC8F}"]),
    ),
    (
        "ARABIC LIGATURE NOON WITH ZAIN",
        ("\u{0646}\u{0632}", ["", "", "", "\u{FC8B}"]),
    ),
    (
        "ARABIC LIGATURE QAF WITH ALEF MAKSURA",
        ("\u{0642}\u{0649}", ["\u{FC35}", "", "", "\u{FC7E}"]),
    ),
    (
        "ARABIC LIGATURE QAF WITH HAH",
        ("\u{0642}\u{062D}", ["\u{FC33}", "\u{FCC2}", "", ""]),
    ),
    (
        "ARABIC LIGATURE QAF WITH MEEM",
        ("\u{0642}\u{0645}", ["\u{FC34}", "\u{FCC3}", "", ""]),
    ),
    (
        "ARABIC LIGATURE QAF WITH MEEM WITH HAH",
        ("\u{0642}\u{0645}\u{062D}", ["", "\u{FDB4}", "", "\u{FD7E}"]),
    ),
    (
        "ARABIC LIGATURE QAF WITH MEEM WITH MEEM",
        ("\u{0642}\u{0645}\u{0645}", ["", "", "", "\u{FD7F}"]),
    ),
    (
        "ARABIC LIGATURE QAF WITH MEEM WITH YEH",
        ("\u{0642}\u{0645}\u{064A}", ["", "", "", "\u{FDB2}"]),
    ),
    (
        "ARABIC LIGATURE QAF WITH YEH",
        ("\u{0642}\u{064A}", ["\u{FC36}", "", "", "\u{FC7F}"]),
    ),
    (
        "ARABIC LIGATURE QALA USED AS KORANIC STOP SIGN",
        ("\u{0642}\u{0644}\u{06D2}", ["\u{FDF1}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE REH WITH SUPERSCRIPT ALEF",
        ("\u{0631}\u{0670}", ["\u{FC5C}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE SAD WITH ALEF MAKSURA",
        ("\u{0635}\u{0649}", ["\u{FD05}", "", "", "\u{FD21}"]),
    ),
    (
        "ARABIC LIGATURE SAD WITH HAH",
        ("\u{0635}\u{062D}", ["\u{FC20}", "\u{FCB1}", "", ""]),
    ),
    (
        "ARABIC LIGATURE SAD WITH HAH WITH HAH",
        ("\u{0635}\u{062D}\u{062D}", ["", "\u{FD65}", "", "\u{FD64}"]),
    ),
    (
        "ARABIC LIGATURE SAD WITH HAH WITH YEH",
        ("\u{0635}\u{062D}\u{064A}", ["", "", "", "\u{FDA9}"]),
    ),
    (
        "ARABIC LIGATURE SAD WITH KHAH",
        ("\u{0635}\u{062E}", ["", "\u{FCB2}", "", ""]),
    ),
    (
        "ARABIC LIGATURE SAD WITH MEEM",
        ("\u{0635}\u{0645}", ["\u{FC21}", "\u{FCB3}", "", ""]),
    ),
    (
        "ARABIC LIGATURE SAD WITH MEEM WITH MEEM",
        ("\u{0635}\u{0645}\u{0645}", ["", "\u{FDC5}", "", "\u{FD66}"]),
    ),
    (
        "ARABIC LIGATURE SAD WITH REH",
        ("\u{0635}\u{0631}", ["\u{FD0F}", "", "", "\u{FD2B}"]),
    ),
    (
        "ARABIC LIGATURE SAD WITH YEH",
        ("\u{0635}\u{064A}", ["\u{FD06}", "", "", "\u{FD22}"]),
    ),
    (
        "ARABIC LIGATURE SALLA USED AS KORANIC STOP SIGN",
        ("\u{0635}\u{0644}\u{06D2}", ["\u{FDF0}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH ALEF MAKSURA",
        ("\u{0633}\u{0649}", ["\u{FCFB}", "", "", "\u{FD17}"]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH HAH",
        ("\u{0633}\u{062D}", ["\u{FC1D}", "\u{FCAE}", "\u{FD35}", ""]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH HAH WITH JEEM",
        ("\u{0633}\u{062D}\u{062C}", ["", "\u{FD5C}", "", ""]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH HEH",
        ("\u{0633}\u{0647}", ["", "\u{FD31}", "\u{FCE8}", ""]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH JEEM",
        ("\u{0633}\u{062C}", ["\u{FC1C}", "\u{FCAD}", "\u{FD34}", ""]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH JEEM WITH ALEF MAKSURA",
        ("\u{0633}\u{062C}\u{0649}", ["", "", "", "\u{FD5E}"]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH JEEM WITH HAH",
        ("\u{0633}\u{062C}\u{062D}", ["", "\u{FD5D}", "", ""]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH KHAH",
        ("\u{0633}\u{062E}", ["\u{FC1E}", "\u{FCAF}", "\u{FD36}", ""]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH KHAH WITH ALEF MAKSURA",
        ("\u{0633}\u{062E}\u{0649}", ["", "", "", "\u{FDA8}"]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH KHAH WITH YEH",
        ("\u{0633}\u{062E}\u{064A}", ["", "", "", "\u{FDC6}"]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH MEEM",
        ("\u{0633}\u{0645}", ["\u{FC1F}", "\u{FCB0}", "\u{FCE7}", ""]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH MEEM WITH HAH",
        ("\u{0633}\u{0645}\u{062D}", ["", "\u{FD60}", "", "\u{FD5F}"]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH MEEM WITH JEEM",
        ("\u{0633}\u{0645}\u{062C}", ["", "\u{FD61}", "", ""]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH MEEM WITH MEEM",
        ("\u{0633}\u{0645}\u{0645}", ["", "\u{FD63}", "", "\u{FD62}"]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH REH",
        ("\u{0633}\u{0631}", ["\u{FD0E}", "", "", "\u{FD2A}"]),
    ),
    (
        "ARABIC LIGATURE SEEN WITH YEH",
        ("\u{0633}\u{064A}", ["\u{FCFC}", "", "", "\u{FD18}"]),
    ),
    // Arabic ligatures with Shadda, the order of characters doesn"t matter
    (
        "ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM",
        (
            "(?:\u{064C}\u{0651}|\u{0651}\u{064C})",
            ["\u{FC5E}", "\u{FC5E}", "\u{FC5E}", "\u{FC5E}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHADDA WITH KASRATAN ISOLATED FORM",
        (
            "(?:\u{064D}\u{0651}|\u{0651}\u{064D})",
            ["\u{FC5F}", "\u{FC5F}", "\u{FC5F}", "\u{FC5F}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHADDA WITH FATHA ISOLATED FORM",
        (
            "(?:\u{064E}\u{0651}|\u{0651}\u{064E})",
            ["\u{FC60}", "\u{FC60}", "\u{FC60}", "\u{FC60}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHADDA WITH DAMMA ISOLATED FORM",
        (
            "(?:\u{064F}\u{0651}|\u{0651}\u{064F})",
            ["\u{FC61}", "\u{FC61}", "\u{FC61}", "\u{FC61}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHADDA WITH KASRA ISOLATED FORM",
        (
            "(?:\u{0650}\u{0651}|\u{0651}\u{0650})",
            ["\u{FC62}", "\u{FC62}", "\u{FC62}", "\u{FC62}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHADDA WITH SUPERSCRIPT ALEF",
        (
            "(?:\u{0651}\u{0670}|\u{0670}\u{0651})",
            ["\u{FC63}", "", "", ""],
        ),
    ),
    // There is a special case when they are with Tatweel
    (
        "ARABIC LIGATURE SHADDA WITH FATHA MEDIAL FORM",
        (
            "\u{0640}(?:\u{064E}\u{0651}|\u{0651}\u{064E})",
            ["\u{FCF2}", "\u{FCF2}", "\u{FCF2}", "\u{FCF2}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHADDA WITH DAMMA MEDIAL FORM",
        (
            "\u{0640}(?:\u{064F}\u{0651}|\u{0651}\u{064F})",
            ["\u{FCF3}", "\u{FCF3}", "\u{FCF3}", "\u{FCF3}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHADDA WITH KASRA MEDIAL FORM",
        (
            "\u{0640}(?:\u{0650}\u{0651}|\u{0651}\u{0650})",
            ["\u{FCF4}", "\u{FCF4}", "\u{FCF4}", "\u{FCF4}"],
        ),
    ),
    // Repeated with different keys to be backward compatible
    (
        "ARABIC LIGATURE SHADDA WITH FATHA",
        (
            "\u{0640}(?:\u{064E}\u{0651}|\u{0651}\u{064E})",
            ["\u{FCF2}", "\u{FCF2}", "\u{FCF2}", "\u{FCF2}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHADDA WITH DAMMA",
        (
            "\u{0640}(?:\u{064F}\u{0651}|\u{0651}\u{064F})",
            ["\u{FCF3}", "\u{FCF3}", "\u{FCF3}", "\u{FCF3}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHADDA WITH KASRA",
        (
            "\u{0640}(?:\u{0650}\u{0651}|\u{0651}\u{0650})",
            ["\u{FCF4}", "\u{FCF4}", "\u{FCF4}", "\u{FCF4}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHEEN WITH ALEF MAKSURA",
        ("\u{0634}\u{0649}", ["\u{FCFD}", "", "", "\u{FD19}"]),
    ),
    (
        "ARABIC LIGATURE SHEEN WITH HAH",
        (
            "\u{0634}\u{062D}",
            ["\u{FD0A}", "\u{FD2E}", "\u{FD38}", "\u{FD26}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHEEN WITH HAH WITH MEEM",
        ("\u{0634}\u{062D}\u{0645}", ["", "\u{FD68}", "", "\u{FD67}"]),
    ),
    (
        "ARABIC LIGATURE SHEEN WITH HAH WITH YEH",
        ("\u{0634}\u{062D}\u{064A}", ["", "", "", "\u{FDAA}"]),
    ),
    (
        "ARABIC LIGATURE SHEEN WITH HEH",
        ("\u{0634}\u{0647}", ["", "\u{FD32}", "\u{FCEA}", ""]),
    ),
    (
        "ARABIC LIGATURE SHEEN WITH JEEM",
        (
            "\u{0634}\u{062C}",
            ["\u{FD09}", "\u{FD2D}", "\u{FD37}", "\u{FD25}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHEEN WITH JEEM WITH YEH",
        ("\u{0634}\u{062C}\u{064A}", ["", "", "", "\u{FD69}"]),
    ),
    (
        "ARABIC LIGATURE SHEEN WITH KHAH",
        (
            "\u{0634}\u{062E}",
            ["\u{FD0B}", "\u{FD2F}", "\u{FD39}", "\u{FD27}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHEEN WITH MEEM",
        (
            "\u{0634}\u{0645}",
            ["\u{FD0C}", "\u{FD30}", "\u{FCE9}", "\u{FD28}"],
        ),
    ),
    (
        "ARABIC LIGATURE SHEEN WITH MEEM WITH KHAH",
        ("\u{0634}\u{0645}\u{062E}", ["", "\u{FD6B}", "", "\u{FD6A}"]),
    ),
    (
        "ARABIC LIGATURE SHEEN WITH MEEM WITH MEEM",
        ("\u{0634}\u{0645}\u{0645}", ["", "\u{FD6D}", "", "\u{FD6C}"]),
    ),
    (
        "ARABIC LIGATURE SHEEN WITH REH",
        ("\u{0634}\u{0631}", ["\u{FD0D}", "", "", "\u{FD29}"]),
    ),
    (
        "ARABIC LIGATURE SHEEN WITH YEH",
        ("\u{0634}\u{064A}", ["\u{FCFE}", "", "", "\u{FD1A}"]),
    ),
    (
        "ARABIC LIGATURE TAH WITH ALEF MAKSURA",
        ("\u{0637}\u{0649}", ["\u{FCF5}", "", "", "\u{FD11}"]),
    ),
    (
        "ARABIC LIGATURE TAH WITH HAH",
        ("\u{0637}\u{062D}", ["\u{FC26}", "\u{FCB8}", "", ""]),
    ),
    (
        "ARABIC LIGATURE TAH WITH MEEM",
        ("\u{0637}\u{0645}", ["\u{FC27}", "\u{FD33}", "\u{FD3A}", ""]),
    ),
    (
        "ARABIC LIGATURE TAH WITH MEEM WITH HAH",
        ("\u{0637}\u{0645}\u{062D}", ["", "\u{FD72}", "", "\u{FD71}"]),
    ),
    (
        "ARABIC LIGATURE TAH WITH MEEM WITH MEEM",
        ("\u{0637}\u{0645}\u{0645}", ["", "\u{FD73}", "", ""]),
    ),
    (
        "ARABIC LIGATURE TAH WITH MEEM WITH YEH",
        ("\u{0637}\u{0645}\u{064A}", ["", "", "", "\u{FD74}"]),
    ),
    (
        "ARABIC LIGATURE TAH WITH YEH",
        ("\u{0637}\u{064A}", ["\u{FCF6}", "", "", "\u{FD12}"]),
    ),
    (
        "ARABIC LIGATURE TEH WITH ALEF MAKSURA",
        ("\u{062A}\u{0649}", ["\u{FC0F}", "", "", "\u{FC74}"]),
    ),
    (
        "ARABIC LIGATURE TEH WITH HAH",
        ("\u{062A}\u{062D}", ["\u{FC0C}", "\u{FCA2}", "", ""]),
    ),
    (
        "ARABIC LIGATURE TEH WITH HAH WITH JEEM",
        ("\u{062A}\u{062D}\u{062C}", ["", "\u{FD52}", "", "\u{FD51}"]),
    ),
    (
        "ARABIC LIGATURE TEH WITH HAH WITH MEEM",
        ("\u{062A}\u{062D}\u{0645}", ["", "\u{FD53}", "", ""]),
    ),
    (
        "ARABIC LIGATURE TEH WITH HEH",
        ("\u{062A}\u{0647}", ["", "\u{FCA5}", "\u{FCE4}", ""]),
    ),
    (
        "ARABIC LIGATURE TEH WITH JEEM",
        ("\u{062A}\u{062C}", ["\u{FC0B}", "\u{FCA1}", "", ""]),
    ),
    (
        "ARABIC LIGATURE TEH WITH JEEM WITH ALEF MAKSURA",
        ("\u{062A}\u{062C}\u{0649}", ["", "", "", "\u{FDA0}"]),
    ),
    (
        "ARABIC LIGATURE TEH WITH JEEM WITH MEEM",
        ("\u{062A}\u{062C}\u{0645}", ["", "\u{FD50}", "", ""]),
    ),
    (
        "ARABIC LIGATURE TEH WITH JEEM WITH YEH",
        ("\u{062A}\u{062C}\u{064A}", ["", "", "", "\u{FD9F}"]),
    ),
    (
        "ARABIC LIGATURE TEH WITH KHAH",
        ("\u{062A}\u{062E}", ["\u{FC0D}", "\u{FCA3}", "", ""]),
    ),
    (
        "ARABIC LIGATURE TEH WITH KHAH WITH ALEF MAKSURA",
        ("\u{062A}\u{062E}\u{0649}", ["", "", "", "\u{FDA2}"]),
    ),
    (
        "ARABIC LIGATURE TEH WITH KHAH WITH MEEM",
        ("\u{062A}\u{062E}\u{0645}", ["", "\u{FD54}", "", ""]),
    ),
    (
        "ARABIC LIGATURE TEH WITH KHAH WITH YEH",
        ("\u{062A}\u{062E}\u{064A}", ["", "", "", "\u{FDA1}"]),
    ),
    (
        "ARABIC LIGATURE TEH WITH MEEM",
        (
            "\u{062A}\u{0645}",
            ["\u{FC0E}", "\u{FCA4}", "\u{FCE3}", "\u{FC72}"],
        ),
    ),
    (
        "ARABIC LIGATURE TEH WITH MEEM WITH ALEF MAKSURA",
        ("\u{062A}\u{0645}\u{0649}", ["", "", "", "\u{FDA4}"]),
    ),
    (
        "ARABIC LIGATURE TEH WITH MEEM WITH HAH",
        ("\u{062A}\u{0645}\u{062D}", ["", "\u{FD56}", "", ""]),
    ),
    (
        "ARABIC LIGATURE TEH WITH MEEM WITH JEEM",
        ("\u{062A}\u{0645}\u{062C}", ["", "\u{FD55}", "", ""]),
    ),
    (
        "ARABIC LIGATURE TEH WITH MEEM WITH KHAH",
        ("\u{062A}\u{0645}\u{062E}", ["", "\u{FD57}", "", ""]),
    ),
    (
        "ARABIC LIGATURE TEH WITH MEEM WITH YEH",
        ("\u{062A}\u{0645}\u{064A}", ["", "", "", "\u{FDA3}"]),
    ),
    (
        "ARABIC LIGATURE TEH WITH NOON",
        ("\u{062A}\u{0646}", ["", "", "", "\u{FC73}"]),
    ),
    (
        "ARABIC LIGATURE TEH WITH REH",
        ("\u{062A}\u{0631}", ["", "", "", "\u{FC70}"]),
    ),
    (
        "ARABIC LIGATURE TEH WITH YEH",
        ("\u{062A}\u{064A}", ["\u{FC10}", "", "", "\u{FC75}"]),
    ),
    (
        "ARABIC LIGATURE TEH WITH ZAIN",
        ("\u{062A}\u{0632}", ["", "", "", "\u{FC71}"]),
    ),
    (
        "ARABIC LIGATURE THAL WITH SUPERSCRIPT ALEF",
        ("\u{0630}\u{0670}", ["\u{FC5B}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE THEH WITH ALEF MAKSURA",
        ("\u{062B}\u{0649}", ["\u{FC13}", "", "", "\u{FC7A}"]),
    ),
    (
        "ARABIC LIGATURE THEH WITH HEH",
        ("\u{062B}\u{0647}", ["", "", "\u{FCE6}", ""]),
    ),
    (
        "ARABIC LIGATURE THEH WITH JEEM",
        ("\u{062B}\u{062C}", ["\u{FC11}", "", "", ""]),
    ),
    (
        "ARABIC LIGATURE THEH WITH MEEM",
        (
            "\u{062B}\u{0645}",
            ["\u{FC12}", "\u{FCA6}", "\u{FCE5}", "\u{FC78}"],
        ),
    ),
    (
        "ARABIC LIGATURE THEH WITH NOON",
        ("\u{062B}\u{0646}", ["", "", "", "\u{FC79}"]),
    ),
    (
        "ARABIC LIGATURE THEH WITH REH",
        ("\u{062B}\u{0631}", ["", "", "", "\u{FC76}"]),
    ),
    (
        "ARABIC LIGATURE THEH WITH YEH",
        ("\u{062B}\u{064A}", ["\u{FC14}", "", "", "\u{FC7B}"]),
    ),
    (
        "ARABIC LIGATURE THEH WITH ZAIN",
        ("\u{062B}\u{0632}", ["", "", "", "\u{FC77}"]),
    ),
    (
        "ARABIC LIGATURE UIGHUR KIRGHIZ YEH WITH HAMZA ABOVE WITH ALEF MAKSURA",
        ("\u{0626}\u{0649}", ["\u{FBF9}", "\u{FBFB}", "", "\u{FBFA}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH ALEF MAKSURA",
        ("\u{064A}\u{0649}", ["\u{FC59}", "", "", "\u{FC95}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAH",
        ("\u{064A}\u{062D}", ["\u{FC56}", "\u{FCDB}", "", ""]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAH WITH YEH",
        ("\u{064A}\u{062D}\u{064A}", ["", "", "", "\u{FDAE}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH AE",
        ("\u{0626}\u{06D5}", ["\u{FBEC}", "", "", "\u{FBED}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH ALEF",
        ("\u{0626}\u{0627}", ["\u{FBEA}", "", "", "\u{FBEB}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH ALEF MAKSURA",
        ("\u{0626}\u{0649}", ["\u{FC03}", "", "", "\u{FC68}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH E",
        ("\u{0626}\u{06D0}", ["\u{FBF6}", "\u{FBF8}", "", "\u{FBF7}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH HAH",
        ("\u{0626}\u{062D}", ["\u{FC01}", "\u{FC98}", "", ""]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH HEH",
        ("\u{0626}\u{0647}", ["", "\u{FC9B}", "\u{FCE0}", ""]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH JEEM",
        ("\u{0626}\u{062C}", ["\u{FC00}", "\u{FC97}", "", ""]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH KHAH",
        ("\u{0626}\u{062E}", ["", "\u{FC99}", "", ""]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH MEEM",
        (
            "\u{0626}\u{0645}",
            ["\u{FC02}", "\u{FC9A}", "\u{FCDF}", "\u{FC66}"],
        ),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH NOON",
        ("\u{0626}\u{0646}", ["", "", "", "\u{FC67}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH OE",
        ("\u{0626}\u{06C6}", ["\u{FBF2}", "", "", "\u{FBF3}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH REH",
        ("\u{0626}\u{0631}", ["", "", "", "\u{FC64}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH U",
        ("\u{0626}\u{06C7}", ["\u{FBF0}", "", "", "\u{FBF1}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH WAW",
        ("\u{0626}\u{0648}", ["\u{FBEE}", "", "", "\u{FBEF}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH YEH",
        ("\u{0626}\u{064A}", ["\u{FC04}", "", "", "\u{FC69}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH YU",
        ("\u{0626}\u{06C8}", ["\u{FBF4}", "", "", "\u{FBF5}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HAMZA ABOVE WITH ZAIN",
        ("\u{0626}\u{0632}", ["", "", "", "\u{FC65}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH HEH",
        ("\u{064A}\u{0647}", ["", "\u{FCDE}", "\u{FCF1}", ""]),
    ),
    (
        "ARABIC LIGATURE YEH WITH JEEM",
        ("\u{064A}\u{062C}", ["\u{FC55}", "\u{FCDA}", "", ""]),
    ),
    (
        "ARABIC LIGATURE YEH WITH JEEM WITH YEH",
        ("\u{064A}\u{062C}\u{064A}", ["", "", "", "\u{FDAF}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH KHAH",
        ("\u{064A}\u{062E}", ["\u{FC57}", "\u{FCDC}", "", ""]),
    ),
    (
        "ARABIC LIGATURE YEH WITH MEEM",
        (
            "\u{064A}\u{0645}",
            ["\u{FC58}", "\u{FCDD}", "\u{FCF0}", "\u{FC93}"],
        ),
    ),
    (
        "ARABIC LIGATURE YEH WITH MEEM WITH MEEM",
        ("\u{064A}\u{0645}\u{0645}", ["", "\u{FD9D}", "", "\u{FD9C}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH MEEM WITH YEH",
        ("\u{064A}\u{0645}\u{064A}", ["", "", "", "\u{FDB0}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH NOON",
        ("\u{064A}\u{0646}", ["", "", "", "\u{FC94}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH REH",
        ("\u{064A}\u{0631}", ["", "", "", "\u{FC91}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH YEH",
        ("\u{064A}\u{064A}", ["\u{FC5A}", "", "", "\u{FC96}"]),
    ),
    (
        "ARABIC LIGATURE YEH WITH ZAIN",
        ("\u{064A}\u{0632}", ["", "", "", "\u{FC92}"]),
    ),
    (
        "ARABIC LIGATURE ZAH WITH MEEM",
        ("\u{0638}\u{0645}", ["\u{FC28}", "\u{FCB9}", "\u{FD3B}", ""]),
    ),
];
