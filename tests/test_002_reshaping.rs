use arabic_reshaper::arabic_reshape;
use arabic_reshaper::letters::LETTERS;
use arabic_reshaper::letters::{FINAL, INITIAL, ISOLATED, MEDIAL, ZWJ};
use arabic_reshaper::ArabicReshaper;

#[test]
fn reshape_test() {
    let cases = [
        ("السلام عليكم", "ﺍﻟﺴﻼﻡ ﻋﻠﻴﻜﻢ"),
        ("السَلَاْمٌ عَلَيْكُمْ", "ﺍﻟﺴﻼﻡ ﻋﻠﻴﻜﻢ"),
        (
            "اللغة العربية هي أكثر اللغات",
            "ﺍﻟﻠﻐﺔ ﺍﻟﻌﺮﺑﻴﺔ ﻫﻲ ﺃﻛﺜﺮ ﺍﻟﻠﻐﺎﺕ",
        ),
        ("تحدثاً ونطقاً ضمن مجموعة", "ﺗﺤﺪﺛﺎ ﻭﻧﻄﻘﺎ ﺿﻤﻦ ﻣﺠﻤﻮﻋﺔ"),
        ("اللغات السامية", "ﺍﻟﻠﻐﺎﺕ ﺍﻟﺴﺎﻣﻴﺔ"),
        ("العربية لغة رسمية في", "ﺍﻟﻌﺮﺑﻴﺔ ﻟﻐﺔ ﺭﺳﻤﻴﺔ ﻓﻲ"),
        ("كل دول الوطن العربي", "ﻛﻞ ﺩﻭﻝ ﺍﻟﻮﻃﻦ ﺍﻟﻌﺮﺑﻲ"),
        ("إضافة إلى كونها لغة", "ﺇﺿﺎﻓﺔ ﺇﻟﻰ ﻛﻮﻧﻬﺎ ﻟﻐﺔ"),
        ("رسمية في تشاد وإريتريا", "ﺭﺳﻤﻴﺔ ﻓﻲ ﺗﺸﺎﺩ ﻭﺇﺭﻳﺘﺮﻳﺎ"),
        ("وإسرائيل. وهي إحدى اللغات", "ﻭﺇﺳﺮﺍﺋﻴﻞ. ﻭﻫﻲ ﺇﺣﺪﻯ ﺍﻟﻠﻐﺎﺕ"),
        ("الرسمية الست في منظمة", "ﺍﻟﺮﺳﻤﻴﺔ ﺍﻟﺴﺖ ﻓﻲ ﻣﻨﻈﻤﺔ"),
        ("الأمم المتحدة، ويُحتفل", "ﺍﻷﻣﻢ ﺍﻟﻤﺘﺤﺪﺓ، ﻭﻳﺤﺘﻔﻞ"),
        ("باليوم العالمي للغة العربية", "ﺑﺎﻟﻴﻮﻡ ﺍﻟﻌﺎﻟﻤﻲ ﻟﻠﻐﺔ ﺍﻟﻌﺮﺑﻴﺔ"),
        ("في 18 ديسمبر كذكرى اعتماد", "ﻓﻲ 18 ﺩﻳﺴﻤﺒﺮ ﻛﺬﻛﺮﻯ ﺍﻋﺘﻤﺎﺩ"),
        ("العربية بين لغات العمل في", "ﺍﻟﻌﺮﺑﻴﺔ ﺑﻴﻦ ﻟﻐﺎﺕ ﺍﻟﻌﻤﻞ ﻓﻲ"),
        ("الأمم المتحدة.", "ﺍﻷﻣﻢ ﺍﻟﻤﺘﺤﺪﺓ."),
    ];
    cases.iter().for_each(|case| {
        assert_eq!(arabic_reshape(case.0), case.1);
    });
}

#[test]
fn test_zwj_reshaping() {
    let beh = 'ب';
    let beh_isolated = LETTERS[&beh][ISOLATED as usize];
    let beh_initial = LETTERS[&beh][INITIAL as usize];
    let beh_medial = LETTERS[&beh][MEDIAL as usize];
    let beh_final = LETTERS[&beh][FINAL as usize];

    let alef = 'ا';
    let alef_final = LETTERS[&alef][FINAL as usize];

    let hamza = 'ء';
    let hamza_isolated = LETTERS[&hamza][ISOLATED as usize];

    let beh = beh.to_string();
    let beh_isolated = beh_isolated.to_string();
    let beh_initial = beh_initial.to_string();
    let beh_final = beh_final.to_string();
    let beh_medial = beh_medial.to_string();
    let hamza = hamza.to_string();
    let hamza_isolated = hamza_isolated.to_string();
    let zwj = ZWJ.to_string();
    let alef = alef.to_string();

    let cases = [
        (beh.clone() + &hamza, beh_isolated.clone() + &hamza_isolated),
        (
            zwj.clone() + &beh + &hamza,
            beh_final.clone() + &hamza_isolated,
        ),
        (zwj.clone() + &beh, beh_final.clone()),
        (beh.clone() + &zwj, beh_initial.clone()),
        (zwj.clone() + &beh + &zwj, beh_medial),
        (
            beh.clone() + &zwj + &hamza,
            beh_initial.clone() + &hamza_isolated,
        ),
        (beh.clone() + &alef, beh_initial.clone() + &alef_final),
        (
            beh.clone() + &zwj + &alef,
            beh_initial.clone() + &alef_final,
        ),
        (
            beh.clone() + &zwj + &alef + &zwj,
            beh_initial.clone() + &alef_final,
        ),
        (
            beh.clone() + &alef + &beh,
            beh_initial.clone() + &alef_final + &beh_isolated,
        ),
        (
            beh.clone() + &zwj + &alef + &zwj + &beh,
            beh_initial.clone() + &alef_final + &beh_final,
        ),
        (
            beh.clone() + &zwj + &hamza + &beh,
            beh_initial.clone() + &hamza_isolated + &beh_isolated,
        ),
        (
            beh.clone() + &zwj + &hamza + &zwj + &beh,
            beh_initial + &hamza_isolated + &beh_final,
        ),
    ];
    cases.iter().for_each(|case| {
        assert_eq!(arabic_reshape(&case.0), case.1);
    });
}
#[test]
fn test_reshaping_with_harakat() {
    let mut reshaper = ArabicReshaper::new();
    *reshaper.configuration.get_mut("delete_harakat").unwrap() = false;

    let cases = [
        ("السَلَاْمٌ عَلَيْكُمْ", "ﺍﻟﺴَﻼَْﻡٌ ﻋَﻠَﻴْﻜُﻢْ"),
        (
            "اللغة العربية هي أكثر اللغات",
            "ﺍﻟﻠﻐﺔ ﺍﻟﻌﺮﺑﻴﺔ ﻫﻲ ﺃﻛﺜﺮ ﺍﻟﻠﻐﺎﺕ",
        ),
        ("تحدثاً ونطقاً ضمن مجموعة", "ﺗﺤﺪﺛﺎً ﻭﻧﻄﻘﺎً ﺿﻤﻦ ﻣﺠﻤﻮﻋﺔ"),
        ("اللغات السامية", "ﺍﻟﻠﻐﺎﺕ ﺍﻟﺴﺎﻣﻴﺔ"),
        ("العربية لغة رسمية في", "ﺍﻟﻌﺮﺑﻴﺔ ﻟﻐﺔ ﺭﺳﻤﻴﺔ ﻓﻲ"),
        ("كل دول الوطن العربي", "ﻛﻞ ﺩﻭﻝ ﺍﻟﻮﻃﻦ ﺍﻟﻌﺮﺑﻲ"),
        ("إضافة إلى كونها لغة", "ﺇﺿﺎﻓﺔ ﺇﻟﻰ ﻛﻮﻧﻬﺎ ﻟﻐﺔ"),
        ("رسمية في تشاد وإريتريا", "ﺭﺳﻤﻴﺔ ﻓﻲ ﺗﺸﺎﺩ ﻭﺇﺭﻳﺘﺮﻳﺎ"),
        ("وإسرائيل. وهي إحدى اللغات", "ﻭﺇﺳﺮﺍﺋﻴﻞ. ﻭﻫﻲ ﺇﺣﺪﻯ ﺍﻟﻠﻐﺎﺕ"),
        ("الرسمية الست في منظمة", "ﺍﻟﺮﺳﻤﻴﺔ ﺍﻟﺴﺖ ﻓﻲ ﻣﻨﻈﻤﺔ"),
        ("الأمم المتحدة، ويُحتفل", "ﺍﻷﻣﻢ ﺍﻟﻤﺘﺤﺪﺓ، ﻭﻳُﺤﺘﻔﻞ"),
        ("باليوم العالمي للغة العربية", "ﺑﺎﻟﻴﻮﻡ ﺍﻟﻌﺎﻟﻤﻲ ﻟﻠﻐﺔ ﺍﻟﻌﺮﺑﻴﺔ"),
        ("في 18 ديسمبر كذكرى اعتماد", "ﻓﻲ 18 ﺩﻳﺴﻤﺒﺮ ﻛﺬﻛﺮﻯ ﺍﻋﺘﻤﺎﺩ"),
        ("العربية بين لغات العمل في", "ﺍﻟﻌﺮﺑﻴﺔ ﺑﻴﻦ ﻟﻐﺎﺕ ﺍﻟﻌﻤﻞ ﻓﻲ"),
        ("الأمم المتحدة.", "ﺍﻷﻣﻢ ﺍﻟﻤﺘﺤﺪﺓ."),
    ];
    cases.iter().for_each(|case| {
        assert_eq!(reshaper.reshape(&case.0), case.1);
    });
}

#[test]
fn test_reshaping_with_harakat_without_ligatures() {
    let mut reshaper = ArabicReshaper::new();
    *reshaper.configuration.get_mut("delete_harakat").unwrap() = false;
    *reshaper.configuration.get_mut("support_ligatures").unwrap() = false;
    let cases = [
        ("السَلَاْمٌ عَلَيْكُمْ", "ﺍﻟﺴَﻠَﺎْﻡٌ ﻋَﻠَﻴْﻜُﻢْ"),
        (
            "اللغة العربية هي أكثر اللغات",
            "ﺍﻟﻠﻐﺔ ﺍﻟﻌﺮﺑﻴﺔ ﻫﻲ ﺃﻛﺜﺮ ﺍﻟﻠﻐﺎﺕ",
        ),
        ("تحدثاً ونطقاً ضمن مجموعة", "ﺗﺤﺪﺛﺎً ﻭﻧﻄﻘﺎً ﺿﻤﻦ ﻣﺠﻤﻮﻋﺔ"),
        ("اللغات السامية", "ﺍﻟﻠﻐﺎﺕ ﺍﻟﺴﺎﻣﻴﺔ"),
        ("العربية لغة رسمية في", "ﺍﻟﻌﺮﺑﻴﺔ ﻟﻐﺔ ﺭﺳﻤﻴﺔ ﻓﻲ"),
        ("كل دول الوطن العربي", "ﻛﻞ ﺩﻭﻝ ﺍﻟﻮﻃﻦ ﺍﻟﻌﺮﺑﻲ"),
        ("إضافة إلى كونها لغة", "ﺇﺿﺎﻓﺔ ﺇﻟﻰ ﻛﻮﻧﻬﺎ ﻟﻐﺔ"),
        ("رسمية في تشاد وإريتريا", "ﺭﺳﻤﻴﺔ ﻓﻲ ﺗﺸﺎﺩ ﻭﺇﺭﻳﺘﺮﻳﺎ"),
        ("وإسرائيل. وهي إحدى اللغات", "ﻭﺇﺳﺮﺍﺋﻴﻞ. ﻭﻫﻲ ﺇﺣﺪﻯ ﺍﻟﻠﻐﺎﺕ"),
        ("الرسمية الست في منظمة", "ﺍﻟﺮﺳﻤﻴﺔ ﺍﻟﺴﺖ ﻓﻲ ﻣﻨﻈﻤﺔ"),
        ("الأمم المتحدة، ويُحتفل", "ﺍﻟﺄﻣﻢ ﺍﻟﻤﺘﺤﺪﺓ، ﻭﻳُﺤﺘﻔﻞ"),
        ("باليوم العالمي للغة العربية", "ﺑﺎﻟﻴﻮﻡ ﺍﻟﻌﺎﻟﻤﻲ ﻟﻠﻐﺔ ﺍﻟﻌﺮﺑﻴﺔ"),
        ("في 18 ديسمبر كذكرى اعتماد", "ﻓﻲ 18 ﺩﻳﺴﻤﺒﺮ ﻛﺬﻛﺮﻯ ﺍﻋﺘﻤﺎﺩ"),
        ("العربية بين لغات العمل في", "ﺍﻟﻌﺮﺑﻴﺔ ﺑﻴﻦ ﻟﻐﺎﺕ ﺍﻟﻌﻤﻞ ﻓﻲ"),
        ("الأمم المتحدة.", "ﺍﻟﺄﻣﻢ ﺍﻟﻤﺘﺤﺪﺓ."),
    ];
    cases.iter().for_each(|case| {
        assert_eq!(reshaper.reshape(&case.0), case.1);
    });
}

#[test]
fn test_reshaping_with_shifted_harakat_without_ligatures() {
    let mut reshaper = ArabicReshaper::new();
    *reshaper.configuration.get_mut("delete_harakat").unwrap() = false;
    *reshaper.configuration.get_mut("support_ligatures").unwrap() = false;
    *reshaper
        .configuration
        .get_mut("shift_harakat_position")
        .unwrap() = true;

    let cases = [("فُعِلَ", "ُﻓِﻌَﻞ"), ("فُعِّلَ", "ُﻓِّﻌَﻞ")];
    cases.iter().for_each(|case| {
        assert_eq!(reshaper.reshape(&case.0), case.1);
    });
}
/*
class TestReshapingWithShiftedHarakatWithoutLigatures(unittest.TestCase):
    def setUp(self):
        self.reshaper = arabic_reshaper.ArabicReshaper({
            "delete_harakat": False,
            "support_ligatures": False,
            "shift_harakat_position": True,
        })
        self.cases = (
            ("فُعِلَ", 'ُﻓِﻌَﻞ"),
            ("فُعِّلَ", 'ُﻓِّﻌَﻞ"),
        )

    def test_reshaping(self):
        _reshaping_test(self)


class TestReshapingSomeLigatures(unittest.TestCase):
    def setUp(self):
        self.reshaper = arabic_reshaper.ArabicReshaper({
            "delete_tatweel": True,
            "ARABIC LIGATURE BISMILLAH AR-RAHMAN AR-RAHEEM": True,
            "ARABIC LIGATURE JALLAJALALOUHOU": True,
            "ARABIC LIGATURE SALLALLAHOU ALAYHE WASALLAM": True,
            "ARABIC LIGATURE ALLAH ": True,
            "ARABIC LIGATURE AKBAR": True,
            "ARABIC LIGATURE ALAYHE": True,
            "ARABIC LIGATURE MOHAMMAD": True,
            "ARABIC LIGATURE RASOUL": True,
            "ARABIC LIGATURE SALAM": True,
            "ARABIC LIGATURE SALLA": True,
            "ARABIC LIGATURE WASALLAM": True,
        })
        self.cases = (
            ("إِنَّهُ مِن سُلَيْمَانَ "
             "وَإِنَّهُ بِسْمِ اللّـَهِ "
             "الرَّحْمَـٰنِ الرَّحِيمِ ﴿٣٠﴾ "
             "أَلَّا تَعْلُوا عَلَيَّ "
             "وَأْتُونِي مُسْلِمِينَ ﴿٣١﴾",

             "ﺇﻧﻪ ﻣﻦ ﺳﻠﻴﻤﺎﻥ ﻭﺇﻧﻪ ﷽ ﴿٣٠﴾ "
             "ﺃﻻ ﺗﻌﻠﻮﺍ ﻋﻠﻲ ﻭﺃﺗﻮﻧﻲ ﻣﺴﻠﻤﻴﻦ ﴿٣١﴾"),

            ("فَذَكِّرْ إِنَّمَا أَنتَ"
             " مُذَكِّرٌ ﴿٢١﴾ لَّسْتَ"
             " عَلَيْهِم بِمُصَيْطِرٍ ﴿٢٢﴾"
             " إِلَّا مَن تَوَلَّىٰ"
             " وَكَفَرَ ﴿٢٣﴾ فَيُعَذِّبُهُ"
             " اللَّـهُ الْعَذَابَ"
             " الْأَكْبَرَ ﴿٢٤﴾",

             "ﻓﺬﻛﺮ ﺇﻧﻤﺎ ﺃﻧﺖ"
             " ﻣﺬﻛﺮ ﴿٢١﴾ ﻟﺴﺖ"
             " ﻋﻠﻴﻬﻢ ﺑﻤﺼﻴﻄﺮ ﴿٢٢﴾"
             " ﺇﻻ ﻣﻦ ﺗﻮﻟﻰ"
             " ﻭﻛﻔﺮ ﴿٢٣﴾ ﻓﻴﻌﺬﺑﻪ"
             " ﷲ ﺍﻟﻌﺬﺍﺏ"
             " ﺍﻷﻛﺒﺮ ﴿٢٤﴾"),

            ("محمد رسول الله صلى الله عليه وسلم",
             "ﷴ ﷶ ﷲ ﷺ"),

            ("الله جل جلاله",
             "ﷲ ﷻ"),

            ("محمد رسول الله عليه صلى الله وسلم",
             "ﷴ ﷶ ﷲ ﷷ ﷹ ﷲ ﷸ"),
        )

    def test_reshaping(self):
        _reshaping_test(self)

if __name__ == "__main__":
    unittest.main()

    */
