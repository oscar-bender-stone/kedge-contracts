// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, FnArg, ItemFn, parse_macro_input};

/// Checks whether a given attribute
/// comes from kedge-contracts.
/// To avoid collisions with other crates,
/// require that the *full path* be used
fn is_kedge_attr(attr: &Attribute, attr_name: &str) -> bool {
    let path = attr.path();

    // Confirm there are exactly two segments,
    // and that the first matche `kedge_contracts`
    if path.segments.len() == 2 {
        let first_segment = &path.segments[0];
        let last_segment = &path.segments[1];

        return first_segment.ident == "kedge_contracts" && last_segment.ident == attr_name;
    }

    false
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
