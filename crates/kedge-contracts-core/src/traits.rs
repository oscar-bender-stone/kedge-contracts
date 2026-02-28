// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use syn::{Expr, ItemFn, Path};

#[derive(Clone)]
pub struct Stub {
    pub original: Path,
    pub replacement: Path,
}
#[allow(unused)]
pub struct BackendOutput {
    pub attrs: Option<Vec<proc_macro2::TokenStream>>,
    pub harness: Option<proc_macro2::TokenStream>,
}

#[allow(unused)]
impl BackendOutput {
    pub fn new(
        attrs: Option<Vec<proc_macro2::TokenStream>>,
        harness: Option<proc_macro2::TokenStream>,
    ) -> Self {
        BackendOutput { attrs, harness }
    }
}

#[allow(unused)]
pub trait Backend {
    /// Generates a specification,
    /// and optionally verification tests
    fn generate(
        input_fn: &ItemFn,
        requires_exprs: &[Expr],
        ensures_exprs: &[Expr],
        stubs: &[Stub],
        is_trusted: bool,
    ) -> BackendOutput;
}
