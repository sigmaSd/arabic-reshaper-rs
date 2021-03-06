**0.4.1**
- Move tests to src. It allows making `letters` module private instead of relying on `#[doc(Hidden)]`

**0.4.0**
- Bump msrv to 1.51 (for const generics)
- Performance improvements
- Removed all dependencies

**0.3.1**
- Remove unneeded character number limitation

**0.3.0**
- Port most of the python tests to rust
- Lots of bug fixes
- ArabicReshaper struct is now added to the public API, it can be used to configure the reshaping algorithm. (see tests directory)

**0.2.1**
- Improve error handling
- Updates dependencies
- Fix clippy warnings

**0.2.0**
- Update to rust 2018 edition
- Update all dependencies
- Switch from lazy_static to once_cell
- Change public API! now this crate only exposes `arabic_reshape`
