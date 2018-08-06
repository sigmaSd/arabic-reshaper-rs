// In the name of Allah

//! # arabic_reshaper
//!
//! Reconstruct Arabic sentences to be used in applications that don't support Arabic script.
//!
//! This crate exposes 2 functions:
//!
//! `arabic_reshape_l` :
//!
//! Reshape letters and reverse their orders so they can be used in left-right context. (you'll probably use most)
//!
//! ```rust
//! extern crate arabic_reshaper;
//! use arabic_reshaper::arabic_reshape_l;
//! let salam = "سلام";
//! println!("{}",arabic_reshape_l(salam));
//! // سلام correctly rendred.
//! ```
//!
//! `arabic_reshape_r`:
//!
//! Only reshapes letters, to be used in a right-left context that doesn't correctly support arabic.
//!
//! ```rust
//! extern crate arabic_reshaper;
//! use arabic_reshaper::arabic_reshape_r;
//! let text = "اللغة العربية رائعة";
//! println!("{}",arabic_reshape_r(text));
//! // الغة العربية رائعة correctly rendred.
//! ```
//! 
//! **More info:**
//! 
//! Check the original python version.
//! 
//! [python-arabic-reshaper](https://github.com/mpcabd/python-arabic-reshaper) 

#[macro_use]
extern crate lazy_static;

mod config_parser;
mod letters;
mod ligatures;

mod algorithm;
use algorithm::ArabicReshaper;

pub fn arabic_reshape_r(text: &str) -> String {
    let mut result = String::new();
    let mut ar = ArabicReshaper::new();

    for line in text.lines() {
        let line = ar.reshape(line.trim());

        result.push_str(&format!("{}\n", line));
    }

    result
}
pub fn arabic_reshape_l(text: &str) -> String {
    let text = arabic_reshape_r(text);
    let mut result: String = text
        .lines()
        .map(|l| l.chars().rev().collect::<String>())
        .map(|l| format!("{}\n", l))
        .collect();

    //get rid of the last \n
    result.pop().unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multi_line() {
        let salam = arabic_reshape_r(
            "السلام
                            عليكم",
        );

        println!("{}", salam);
        let salam = arabic_reshape_l(
            "السلام
                            عليكم",
        );

        println!("{}", salam);
    }

}
