// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

#![allow(unused)]
extern crate flux_rs;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, FnArg, ItemFn, parse_macro_input};
use syn::{Pat, ReturnType};

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
pub fn generate_flux_attr(
    input_fn: &ItemFn,
    requires: &[Expr],
    ensures: &[Expr],
) -> proc_macro2::TokenStream {
    // flux_rs::spec expects a string as argument,
    // so we must construct it manually
    let mut sig_string = String::from("fn(");

    for (i, input) in input_fn.sig.inputs.iter().enumerate() {
        if i > 0 {
            sig_string.push_str(", ");
        }

        match input {
            FnArg::Typed(pat_type) => {
                // Extract argument name
                let pat_str = match &*pat_type.pat {
                    Pat::Ident(p) => p.ident.to_string(),
                    _ => "_".to_string(), // For now, create a placeholder for patterns
                };

                let ty_str = quote::quote!(#pat_type.ty).to_string().replace(" ", " ");

                sig_string.push_str(&format!("{}: {}", pat_str, ty_str));
            }
            FnArg::Receiver(_) => {
                // Handle 'self'
                sig_string.push_str("self");
            }
        }
    }

    sig_string.push_str(") -> ");

    // Handle return type
    match &input_fn.sig.output {
        ReturnType::Default => sig_string.push_str("()"),
        ReturnType::Type(_, ty) => {
            let ret_str = quote::quote!(#ty).to_string().replace(" ", "");
            sig_string.push_str(&ret_str);
        }
    }

    // To make the translation easier,
    // use 'requires` clauses
    for req in requires {
        let req_str = quote::quote!(#req).to_string();
        sig_string.push_str(&format!(" requires {}", req_str));
    }

    // Now append `ensures` clauses.
    // Note that we must bind to the return value, e.g.,
    // " ensures result: result > x"
    for ens in ensures {
        let ens_str = quote::quote!(#ens).to_string();
        // TODO: enforce result being used in expression
        sig_string.push_str(&format!(" ensures result: {}", ens_str));
    }

    quote! {
        #[cfg_attr(flux, flux::spec(fn(#sig_string)))]
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

    let attrs = &input_fn.attrs;
    let flux_spec = generate_flux_attr(&input_fn, &requires_exprs, &ensures_exprs);

    quote! {
        #(#attrs)*
        #flux_spec
        #input_fn
    }
    .into()
}
