# arabic-reshaper-rs

Reconstruct Arabic sentences to be used in applications that don't support Arabic.

This crate is [python-arabic-reshaper](https://github.com/mpcabd/python-arabic-reshaper) ported to rust. 

## Usage:

This crate exposes 2 functions:

`arabic_reshape_l`:

Reshape letters and reverse their orders so they can be used in left-right context. (you'll probably use most)

```rust
extern crate arabic_reshaper;
use arabic_reshaper::arabic_reshape_l;
let salam = "سلام";
println!("{}",arabic_reshape_l(salam));
// سلام correctly rendred.
```

`arabic_reshape_r`:

Only reshapes letters, to be used in a right-left context that doesn't correctly support arabic.

```rust
extern crate arabic_reshaper;
use arabic_reshaper::arabic_reshape_r;
let text = "اللغة العربية رائعة";
println!("{}",arabic_reshape_r(text));
// الغة العربية رائعة correctly rendred.
```

**Credits/More info:**

Check the awesome original python version.

[python-arabic-reshaper](https://github.com/mpcabd/python-arabic-reshaper)

