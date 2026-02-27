// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

#[cfg(kani)]
pub use kani;

use kedge_contracts_core::traits::{Backend, BackendOutput};
use quote::quote;
use syn::{Expr, FnArg, ItemFn};

pub(crate) struct KaniBackend;

impl Backend for KaniBackend {
    fn generate(
        input_fn: &ItemFn,
        requires_exprs: &[Expr],
        ensures_exprs: &[Expr],
    ) -> BackendOutput {
        let fn_name = &input_fn.sig.ident;
        let mut arg_decls = Vec::new();
        let mut arg_names = Vec::new();

        for arg in &input_fn.sig.inputs {
            if let FnArg::Typed(pat_type) = arg {
                let pat = &pat_type.pat;
                let ty = &pat_type.ty;
                arg_decls.push(quote! {
                    let #pat: #ty = kani::any();
                });
                arg_names.push(pat);
            }
        }

        let mut kani_attrs = Vec::new();

        let kani_requires: Vec<proc_macro2::TokenStream> = requires_exprs
            .iter()
            .map(|expr| {
                quote! { #[cfg_attr(kani, ::kani::requires(#expr))] }
            })
            .collect();

        kani_attrs.extend(kani_requires);

        // To pass on conditions to Kani more easily,
        // wrap the expression around a closure |result| { ... }
        let kani_ensures: Vec<proc_macro2::TokenStream> = ensures_exprs
            .iter()
            .map(|expr| {
                quote! { #[cfg_attr(kani, ::kani::ensures(|result| { #expr }))] }
            })
            .collect();

        kani_attrs.extend(kani_ensures);

        // Call the function,
        // and make `result` a refernece
        // to work with kani::ensures

        let harness_name = quote::format_ident!("__harness_{}", fn_name);

        let kani_harness = quote! {
            #[cfg(kani)]
            #[kani::proof]
            #[allow(dead_code)]
            fn #harness_name() {
                #(#arg_decls)*

                #(kani::assume(#requires_exprs);)*

                let temp_result = #fn_name(#(#arg_names),*);

                let result = &temp_result;
            }
        };

        BackendOutput::new(Some(kani_attrs), Some(kani_harness))
    }
}
