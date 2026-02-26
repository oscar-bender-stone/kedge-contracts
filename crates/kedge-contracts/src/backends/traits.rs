// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use syn::{Expr, ItemFn};

trait Backend {
    /// Generates a specification, and optionally verification tests
    fn generate(input_fn: &ItemFn, requires: &[Expr], ensures: &[Expr])
    -> proc_macro2::TokenStream;
}
