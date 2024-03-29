# arabic-reshaper-rs

Reconstruct Arabic sentences to be used in applications that don't support Arabic.

This crate is [python-arabic-reshaper](https://github.com/mpcabd/python-arabic-reshaper) ported to rust.

Like `python-arabic-reshaper` it reshapes the letters, but it doesnt reverse the writing, the user is responsible for that: you can look at [Unicode bidirectional algorithm](http://unicode.org/reports/tr9/) or simply try reversing the characters orders and see if that works for you

## Usage:

`arabic_reshape`:

Reshape letters

```rust
use arabic_reshaper::arabic_reshape;
let salam = "سلام";
println!("{}",arabic_reshape(salam));
//  سلام is now correctly reshaped but the letters are not reversed yet
// For simple cases we can  use chars().rev()
// For a more correct approach http://unicode.org/reports/tr9/ should be used (via https://docs.rs/unic_bidi crate for example)
```

Checkout the tests/bench directories for more examples.

## Minimum Rust version
Starting version `0.4.0` the msrv is  **1.51**

### CLI:

[`rtl_reshaper_rs`](https://github.com/NightMachinary/rtl_reshaper_rs) is a CLI frontend available that reshapes and reorders `stdin`.

## Todo:

- [ ] document the code

**Credits/More info:**

Check the awesome original python version.

[python-arabic-reshaper](https://github.com/mpcabd/python-arabic-reshaper)
