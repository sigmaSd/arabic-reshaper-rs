# arabic-reshaper-rs

Reconstruct Arabic sentences to be used in applications that don't support Arabic.

This crate is [python-arabic-reshaper](https://github.com/mpcabd/python-arabic-reshaper) ported to rust.

## Usage:

`arabic_reshape`:

Reshape letters

```rust
use arabic_reshaper::arabic_reshape;
let salam = "سلام";
println!("{}",arabic_reshape(salam));
// سلام correctly rendred.
```

## Todo:

- [ ] document the code

**Credits/More info:**

Check the awesome original python version.

[python-arabic-reshaper](https://github.com/mpcabd/python-arabic-reshaper)
