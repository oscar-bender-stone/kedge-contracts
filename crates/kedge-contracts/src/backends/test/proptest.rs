// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

#[cfg(feature = "proptest")]
pub extern crate proptest;

use kedge_contracts_core::traits::{Backend, BackendOutput};
use quote::quote;

pub struct ProptestBackend;

impl Backend for ProptestBackend {
    fn generate(
        input_fn: &syn::ItemFn,
        requires_exprs: &[syn::Expr],
        ensures_exprs: &[syn::Expr],
    ) -> kedge_contracts_core::traits::BackendOutput {
        let fn_vis = &input_fn.vis;
        let fn_name = &input_fn.sig.ident;

        let harness_sig = &mut input_fn.sig.clone();

        let harness_name = quote::format_ident!("___harness_{}", fn_name);

        harness_sig.ident = harness_name;
        harness_sig.output = syn::ReturnType::Default;

        let proptest_assumes = if requires_exprs.is_empty() {
            quote! {}
        } else {
            quote! { #(proptest::prop_assumes!(#requires_exprs);)* }
        };

        let proptest_asserts = if ensures_exprs.is_empty() {
            quote! {}
        } else {
            quote! { #(proptest::prop_assumes!(#ensures_exprs);)* }
        };

        let proptest_harness = quote! {
            #[cfg(feature = "proptest")]
            #[property_test]
            #harness_sig {
                #proptest_assumes
                #proptest_asserts
            }
        };

        BackendOutput::new(None, Some(proptest_harness))
    }
}
