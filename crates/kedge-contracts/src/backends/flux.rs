// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

#![allow(unused)]
extern crate flux_rs;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, FnArg, ItemFn, parse_macro_input};

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

// TODO: add support for structs and enums
// TODO: support more complex conditions
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

    let attrs = &input_fn.attrs;
    let fn_sig = &input_fn.sig;

    for arg in &input_fn.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            let pat = &pat_type.pat;
            let ty = &pat_type.ty;
        }
    }

    todo!();

    quote! {
        #(#attrs)*
        #input_fn
    }
    .into()
}
