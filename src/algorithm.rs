use crate::config_parser::parse;
use crate::letters::*;
use crate::ligatures::LIGATURES;

use std::collections::HashMap;
use std::iter::repeat;

static HARAKAT_RE: [std::ops::Range<char>; 9] = [
    '\u{0610}'..'\u{061a}',
    '\u{064b}'..'\u{065f}',
    '\u{0670}'..'\u{0670}',
    '\u{06d6}'..'\u{06dc}',
    '\u{06df}'..'\u{06e8}',
    '\u{06ea}'..'\u{06ed}',
    '\u{08d4}'..'\u{08e1}',
    '\u{08d4}'..'\u{08ed}',
    '\u{08e3}'..'\u{08ff}',
];

const NOT_SUPPORTED: i16 = -1;
const EMPTY: (CharType, i16) = (CharType::Unsupported, NOT_SUPPORTED);

#[derive(Copy, Clone)]
enum CharType {
    Supported(char),
    Unsupported,
}

impl From<char> for CharType {
    fn from(c: char) -> Self {
        CharType::Supported(c)
    }
}

pub struct ArabicReshaper<'a> {
    pub configuration: HashMap<String, bool>,
    re_group_index_to_ligature_forms: HashMap<usize, [&'static str; 4]>,
    patterns: Vec<&'a str>,
}

impl Default for ArabicReshaper<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl ArabicReshaper<'_> {
    pub fn new() -> Self {
        Self {
            configuration: parse(),
            re_group_index_to_ligature_forms: HashMap::new(),
            patterns: Vec::new(),
        }
    }

    #[allow(clippy::cognitive_complexity)]
    pub fn reshape(&mut self, text: &str) -> String {
        let mut output = Vec::new();

        let delete_harakat = self.configuration["delete_harakat"];
        let delete_tatweel = self.configuration["delete_tatweel"];
        let support_zwj = self.configuration["support_zwj"];
        let shift_harakat_position = self.configuration["shift_harakat_position"];
        let use_unshaped_instead_of_isolated =
            self.configuration["use_unshaped_instead_of_isolated"];

        let mut position_harakat: HashMap<i16, Vec<char>> = HashMap::new();

        let isolated_form = if use_unshaped_instead_of_isolated {
            UNSHAPED
        } else {
            ISOLATED
        };

        for letter in text.chars() {
            if HARAKAT_RE.iter().any(|r| r.contains(&letter)) {
                let len = output.len() as i16;
                if !delete_harakat {
                    let mut position = len - 1;
                    if shift_harakat_position {
                        position -= 1;
                    }

                    position_harakat.entry(position).or_insert_with(Vec::new);

                    if shift_harakat_position {
                        let v = position_harakat.entry(position).or_default();
                        v.insert(0, letter);
                    } else {
                        let v = position_harakat.entry(position).or_default();
                        v.push(letter);
                    }
                }
            } else if letter == TATWEEL && delete_tatweel || letter == ZWJ && !support_zwj {
            } else if !LETTERS.contains_key(&letter) {
                output.push((letter, NOT_SUPPORTED));
            } else if output.is_empty() {
                output.push((letter, isolated_form));
            } else {
                let len = output.len();
                let out_clone = output.clone();
                let previous_letter = out_clone[len - 1];
                if (previous_letter.1 == NOT_SUPPORTED)
                    || (!connects_with_letter_before(letter))
                    || (!connects_with_letter_after(previous_letter.0))
                    || (previous_letter.1 == FINAL
                        && !connects_with_letters_before_and_after(previous_letter.0))
                {
                    output.push((letter, isolated_form));
                } else if previous_letter.1 == isolated_form {
                    output[len - 1] = (previous_letter.0, INITIAL);
                    output.push((letter, FINAL));
                } else {
                    output[len - 1] = (previous_letter.0, MEDIAL);
                    output.push((letter, FINAL));
                }
            }
            let len = output.len();
            if support_zwj && output.len() > 1 && output[len - 2].0 == ZWJ {
                output.remove(len - 2);
            }
        }

        if support_zwj && !output.is_empty() && output[output.len() - 1].0 == ZWJ {
            output.pop();
        }

        let mut output: Vec<(CharType, i16)> = output
            .into_iter()
            .map(|(c, i)| (CharType::from(c), i))
            .collect();

        if self.configuration["support_ligatures"] {
            let mut text: String = text
                .chars()
                .filter(|c| !HARAKAT_RE.iter().any(|r| r.contains(c)))
                .collect();

            if delete_tatweel {
                text = text.replace(TATWEEL, "");
            }

            // fill patterns
            self.ligature_re();

            // find patter matches in text
            for re_match in captures_iter(&text, &self.patterns) {
                let group_index = re_match.pattern_idx;

                //regex returns bytes offset
                //we want character position
                let a = text[..re_match.start].chars().count();
                let b = text[..re_match.end].chars().count();

                let forms = self.re_group_index_to_ligature_forms[&group_index];

                let a_form = output[a].1;
                let b_form = output[b - 1].1;
                let ligature_form;

                // +-----------+----------+---------+---------+----------+
                // | a   \   b | ISOLATED | INITIAL | MEDIAL  | FINAL    |
                // +-----------+----------+---------+---------+----------+
                // | ISOLATED  | ISOLATED | INITIAL | INITIAL | ISOLATED |
                // | INITIAL   | ISOLATED | INITIAL | INITIAL | ISOLATED |
                // | MEDIAL    | FINAL    | MEDIAL  | MEDIAL  | FINAL    |
                // | FINAL     | FINAL    | MEDIAL  | MEDIAL  | FINAL    |
                // +-----------+----------+---------+---------+----------+

                if a_form == isolated_form || a_form == INITIAL {
                    if b_form == isolated_form || b_form == FINAL {
                        ligature_form = ISOLATED;
                    } else {
                        ligature_form = INITIAL;
                    }
                } else if b_form == isolated_form || b_form == FINAL {
                    ligature_form = FINAL;
                } else {
                    ligature_form = MEDIAL;
                }

                if forms[ligature_form as usize].is_empty() {
                    continue;
                }
                output[a] = (
                    forms[ligature_form as usize].chars().next().unwrap().into(),
                    NOT_SUPPORTED,
                );
                let v: Vec<(CharType, i16)> = repeat(EMPTY).take(b - 1 - a).collect();

                let mut index = a + 1;
                let mut v_index = 0;
                while index != b {
                    output[index] = v[v_index];
                    index += 1;
                    v_index += 1;
                }
            }
        }
        let mut result = Vec::new();
        if !delete_harakat && position_harakat.contains_key(&-1) {
            result.extend(position_harakat[&-1].clone());
        }
        for (i, o) in output.iter().enumerate() {
            if let CharType::Supported(c) = o.0 {
                if o.1 == NOT_SUPPORTED || o.1 == UNSHAPED {
                    result.push(c);
                } else {
                    let unc = LETTERS[c][o.1 as usize];
                    result.push(unc.chars().next().unwrap());
                }
            }
            if !delete_harakat && position_harakat.contains_key(&(i as i16)) {
                result.extend(position_harakat[&(i as i16)].clone());
            }
        }

        let result: String = result.into_iter().collect();
        result
    }

    fn ligature_re(&mut self) {
        let mut index = 0;
        //const FORMS: i16 = 1;
        //const MATCH: i16 = 0;
        if self.re_group_index_to_ligature_forms.is_empty() {
            for ligature_record in LIGATURES.iter() {
                let (ligature, replacement) = ligature_record;
                if !self.configuration[*ligature] {
                    continue;
                }

                self.re_group_index_to_ligature_forms
                    .insert(index, replacement.1);
                self.patterns.push(replacement.0);
                index += 1;
            }
        }
    }
}

fn captures_iter(s: &str, patterns: &[&str]) -> Vec<Match> {
    let mut result = vec![];
    for (pattern_idx, pat) in patterns.iter().enumerate() {
        let start = match s.find(pat) {
            Some(idx) => idx,
            None => {
                continue;
            }
        };

        let end = start + pat.len();
        result.push(Match {
            start,
            end,
            pattern_idx,
        });
    }
    result
}

#[test]
fn test_captrues_iter() {
    let test = "aaa__sqdbbbdbb";
    let patterns = vec!["aaa", "bbb"];

    assert_eq!(
        captures_iter(test, &patterns),
        vec!(
            Match {
                start: 0,
                end: 3,
                pattern_idx: 0
            },
            Match {
                start: 8,
                end: 11,
                pattern_idx: 1
            }
        )
    );
}

#[derive(Debug, PartialEq)]
struct Match {
    start: usize,
    end: usize,
    pattern_idx: usize,
}
