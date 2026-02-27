// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use kedge_contracts_core::traits::{Backend, BackendOutput};
use quote::quote;

pub struct ProptestBackend;

impl Backend for ProptestBackend {
    fn generate(
        input_fn: &syn::ItemFn,
        requires_exprs: &[syn::Expr],
        ensures_exprs: &[syn::Expr],
    ) -> kedge_contracts_core::traits::BackendOutput {
        let fn_name = &input_fn.sig.ident;
        let fn_sig = &input_fn.sig;

        let harness_name = quote::format_ident!("__harness_{}", fn_name);

        let proptest_assumes = if requires_exprs.is_empty() {
            quote! {}
        } else {
            quote! { #(::proptest::prop_assume!(#),*) }
        };

        let proptest_asserts = if ensures_exprs.is_empty() {
            quote! {}
        } else {
            quote! { #(::proptest::prop_assert!(#),*) }
        };

        let proptest_harness = quote! {
        const _: () = {
            #[property_test]
            fn #harness_name #fn_sig {
                #proptest_assumes
                #proptest_asserts
            }
        }
        };

        BackendOutput::new(None, Some(proptest_harness))
    }
}
