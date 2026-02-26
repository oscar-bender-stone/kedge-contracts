// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

mod backends;
mod validate;

use crate::validate::is_kedge_attr;
use backends::verify::{flux::FluxBackend, kani::KaniBackend};
use kedge_contracts_core::traits::Backend;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ItemFn, parse_macro_input};

#[proc_macro_attribute]
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

    let kani_output = KaniBackend::generate(&input_fn, &requires_exprs, &ensures_exprs);
    let flux_output = FluxBackend::generate(&input_fn, &requires_exprs, &ensures_exprs);

    let mut contract_attrs = Vec::new();

    if let Some(attrs) = kani_output.attrs {
        contract_attrs.extend(attrs)
    }

    if let Some(attrs) = flux_output.attrs {
        contract_attrs.extend(attrs)
    }

    let harnesses = vec![kani_output.harness];

    quote! {
        #(#contract_attrs)*
        #input_fn
        #(#harnesses)*
    }
    .into()
}

/// A marker that *cannot* be used alone.
/// If applied without `kedge_contracts::contract`,
/// this function will error.
#[proc_macro_attribute]
pub fn requires(_conditions: TokenStream, _input_fn: TokenStream) -> TokenStream {
    quote! {
        compile_error!(
            "The `#[requires]` attribute cannot be used alone. \n\
             You must add `#[kedge_contracts::contract]` to the function.\n\
             Example:\n   \
                 #[kedge_contracts::contract]\n   \
                 #[kedge_contracts::requires(x > 0)]\n   \
                 fn my_func(x: i8) ..."
        );
    }
    .into()
}

/// A marker that *cannot* be used alone.
/// If applied without `kedge_contracts::contract`,
/// this function will error.
#[proc_macro_attribute]
pub fn ensures(_conditions: TokenStream, _input_fn: TokenStream) -> TokenStream {
    quote! {
        compile_error!(
            "The `#[ensures]` attribute cannot be used alone. \n\
             You must add `#[kedge_contracts::contract]` to the function.\n\
             Example:\n   \
                 #[kedge_contracts::contract]\n   \
                 #[kedge_contracts::ensures(x > 0)]\n   \
                 fn my_func(x: i8) ..."
        );
    }
    .into()
}
