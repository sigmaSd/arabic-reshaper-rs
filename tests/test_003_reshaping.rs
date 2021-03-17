use arabic_reshaper::arabic_reshape;
#[test]
fn test_default_reshaping() {
    let cases = [
        ("چۆمان", "ﭼﯚﻣﺎﻥ"),
        ("گۆیژە", "ﮔﯚﯾﮋە"),
        ("ﺧﯚﻣﺎﻥ ﺧﯚﺵ", "ﺧﯚﻣﺎﻥ ﺧﯚﺵ"),
    ];
    cases.iter().for_each(|case| {
        assert_eq!(arabic_reshape(case.0), case.1);
    });
}
