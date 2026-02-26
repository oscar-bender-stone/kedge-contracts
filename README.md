<!-- SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com> -->
<!-- SPDX-License-Identifier: MIT -->

# Kedge (WIP)

**WARNING: This is heavily in development! Expect breaking changes!**

Tired of writing tests cases? Want to run verification tools on your crate? Or,
even more crazy, _synthesize_ your code? Kedge offers it all. Choose your level
of automation best suited for your project.

Note that you can keep your existing tests, but if you're looking to _create_
your own, kedge doesn't provide a testing framework itself. For that, there are
[several existing options](https://github.com/rust-unofficial/awesome-rust?tab=readme-ov-file#testing)
(credit to the contributors of `unofficial-rust/awesome-rust`!).

## Quick Start

### Testing Existing Code

To test your existing code, you'll want to add the `contracts` feature, enabled
by default. From there, you can choose to use:

- `test`: automatically generate cases based on your contracts!

- `verify`: verify your contracts using tools from formal methods. For more
  information, please see:
  [Rust Formal Methods Group](https://rust-formal-methods.github.io/).

[TBD] To try all tests, run:

```shell
cargo kedge
```

### Synthesizing Code

To generate code, enable the `synth`. For generating _specifications_ as well,
use `spec`.

## License

2026 (c) Oscar Bender-Stone <oscar-bender-stone@protonmail.com>

MIT License.

Where noted, .gitignore, build artifacts, etc.), CC0-1.0 applies.
