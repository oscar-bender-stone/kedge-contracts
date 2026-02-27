<!-- SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com> -->
<!-- SPDX-License-Identifier: MIT -->

# Kedge Contracts (WIP)

**WARNING: This is heavily in development! Expect breaking changes!**

Tired of writing tests cases? Want to run verification tools on your crate? Then
Kedge is the tool for you! Choose your level of automation best suited for your
project.

Note that you can keep your existing tests, but if you're looking to _create_
your own, kedge doesn't provide a testing framework itself. For that, there are
[several existing options](https://github.com/rust-unofficial/awesome-rust?tab=readme-ov-file#testing)
(credit to the contributors of `unofficial-rust/awesome-rust`!).

## Quick Start

### Testing Existing Code

To test your existing code, you'll want to add the `contracts` feature, enabled
by default. From there, you can choose to use:

- `test`: automatically generate cases based on your contracts!
  - `proptest`: use the
    [proptest backend](https://github.com/proptest-rs/proptest) via
    `cargo test`.

- `verify`: verify your contracts using tools from formal methods. For more
  information, please see:
  [Rust Formal Methods Group](https://rust-formal-methods.github.io/).
  - `kani`: model checker for Rust. Useful for checking **bounded** low-level
    bit operations or unsafe blocks. Make sure to
    [install kani](https://model-checking.github.io/kani/install-guide.html)).
    Then, run: `cargo kani`.

  - `flux`: refinement types in Rust. Useful to define specific types, e.g.,
    `i32[10]` to have _exactly_ the `i32` with value `10`. Make sure to
    [install flux](https://flux-rs.github.io/flux/guide/install.html)). Then,
    run `cargo flux`.

## License

2026 (c) Oscar Bender-Stone <oscar-bender-stone@protonmail.com>

MIT License.

Where noted, .gitignore, build artifacts, etc.), CC0-1.0 applies.
