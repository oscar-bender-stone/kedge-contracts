// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use kedge_contracts_core::traits::{Backend, BackendOutput};
use quote::quote;

use syn::{FnArg, Pat};

pub struct ProptestBackend;

impl Backend for ProptestBackend {
    fn generate(
        input_fn: &syn::ItemFn,
        requires_exprs: &[syn::Expr],
        ensures_exprs: &[syn::Expr],
    ) -> kedge_contracts_core::traits::BackendOutput {
        let fn_name = &input_fn.sig.ident;

        let mut call_args = Vec::new();

        for input in &input_fn.sig.inputs {
            if let FnArg::Typed(pat_type) = input {
                match &*pat_type.pat {
                    // Easy case: just get the ident
                    Pat::Ident(pat_ident) => call_args.push(quote! { #pat_ident }),
                    // Harder case: need to consider
                    // arguments like
                    // (a, b): (i8, i8) or Struct { x }: Struct.
                    // Pass whole pattern to harness
                    // and extract internal names
                    _ => {
                        let pat = &pat_type.pat;
                        call_args.push(quote! { #pat });
                    }
                }
            } else if let FnArg::Receiver(_) = input {
                call_args.push(quote! { self });
            }
        }

        let harness_sig = &mut input_fn.sig.clone();

        let harness_name = quote::format_ident!("___harness_{}", fn_name);

        harness_sig.ident = harness_name;
        harness_sig.output = syn::ReturnType::Default;

        let proptest_assumes = if requires_exprs.is_empty() {
            quote! {}
        } else {
            quote! { #(::proptest::prop_assume!(#requires_exprs);)* }
        };

        let proptest_asserts = if ensures_exprs.is_empty() {
            quote! {}
        } else {
            quote! { #(::proptest::prop_assert!(#ensures_exprs);)* }
        };

        let proptest_harness = quote! {
            ::proptest::proptest! {
                #[test]
                #harness_sig {
                    #proptest_assumes
                    let result = &(#fn_name(#(#call_args),*));
                    #proptest_asserts
                }
            }
        };

        println!("{proptest_harness}");

        BackendOutput::new(None, Some(proptest_harness))
    }
}
