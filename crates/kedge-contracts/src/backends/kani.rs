// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use super::traits::{Backend, BackendOutput};
use crate::validate::is_kedge_attr;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, FnArg, ItemFn, parse_macro_input};

struct KaniBackend;

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
                quote! { #[cfg_attr(kani, kani::requires(#expr))] }
            })
            .collect();

        kani_attrs.extend(kani_requires);

        // To pass on conditions to Kani more easily,
        // wrap the expression around a closure |result| { ... }
        let kani_ensures: Vec<proc_macro2::TokenStream> = ensures_exprs
            .iter()
            .map(|expr| {
                quote! { #[cfg_attr(kani, kani::ensures(|result| { #expr }))] }
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

pub fn contract(_args: TokenStream, input_fn: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(input_fn as ItemFn);
    let mut requires_exprs = Vec::new();
    let mut ensures_exprs = Vec::new();

    // Filter attributes
    input_fn.attrs.retain(|attr| {
        if is_kedge_attr(attr, "requires") {
            if let Ok(expr) = attr.parse_args::<Expr>() {
                requires_exprs.push(expr);
            }
            false
        } else if is_kedge_attr(attr, "ensures") {
            if let Ok(expr) = attr.parse_args::<Expr>() {
                ensures_exprs.push(expr);
            }
            false
        } else {
            true
        }
    });

    let fn_name = &input_fn.sig.ident;
    let attrs = &input_fn.attrs;

    // Generate Kani attributes.
    // To pass on conditions to Kani more easily,
    // wrap the expression around a closure |result| { ... }
    let kani_requires = requires_exprs.iter().map(|expr| {
        quote! { #[cfg_attr(kani, kani::requires(#expr))] }
    });

    let kani_ensures = ensures_exprs.iter().map(|expr| {
        quote! { #[cfg_attr(kani, kani::ensures(|result| { #expr }))] }
    });

    // Setup arguments for harness
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

    // 4. Harness Generation
    let harness_name = quote::format_ident!("__harness_{}", fn_name);

    // Call the function,
    // and make `result` a refernece
    // to work with kani::ensures
    let harness = quote! {
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

    quote! {
        #(#attrs)*
        #(#kani_requires)*
        #(#kani_ensures)*
        #input_fn
        #harness
    }
    .into()
}
