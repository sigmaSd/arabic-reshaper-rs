// In the name of Allah

//! # arabic_reshaper
//!
//! Reconstruct Arabic sentences to be used in applications that don't support Arabic script.
//!
//! `arabic_reshape` :
//!
//! Reshape letters
//!
//! ```rust
//! use arabic_reshaper::arabic_reshape;
//! let salam = "سلام";
//! println!("{}",arabic_reshape(salam));
//! // سلام correctly rendred.
//! ```
//!
//! **More info:**
//!
//! Check the original python version.
//!
//! [python-arabic-reshaper](https://github.com/mpcabd/python-arabic-reshaper)

mod config_parser;
pub mod letters;
mod ligatures;

mod algorithm;
pub use algorithm::ArabicReshaper;

pub fn arabic_reshape(text: &str) -> String {
    let mut ar = ArabicReshaper::new();

    ar.reshape(text)
}
