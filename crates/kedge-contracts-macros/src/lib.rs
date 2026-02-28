// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

mod backends;
mod validate;

use crate::backends::test::proptest::ProptestBackend;
use crate::validate::is_kedge_attr;
use backends::verify::{flux::FluxBackend, kani::KaniBackend};
use kedge_contracts_core::traits::{Backend, BackendOutput};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ItemFn, parse_macro_input};

macro_rules! collect_backends {
    ([ $($backend:path),* $(,)? ]) => {
        |target: &mut Vec<BackendOutput>, input: &ItemFn, req: &[Expr], ens: &[Expr], is_trusted: bool| {
            $(
                target.push(<$backend as Backend>::generate(input, req, ens, is_trusted));
            )*
        }
    };
}

// WARNING: the prefix `__harness` is reserved.
// Because some backends uses `#[test]`,
// there isn't a great way to guarantee
// isolation. Please keep this in mind
// and *avoid* using the prefix `_harness`
// in your function names.
#[proc_macro_attribute]
pub fn contract(args: TokenStream, input_fn: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(input_fn as ItemFn);
    let mut requires_exprs = Vec::new();
    let mut ensures_exprs = Vec::new();
    let mut is_trusted = false;

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
        } else if is_kedge_attr(attr, "trusted") {
            is_trusted = true;
            false
        } else {
            true
        }
    });

    let mut backend_outputs = Vec::new();

    let run_backends = if args.is_empty() {
        collect_backends!([KaniBackend, FluxBackend, ProptestBackend])
    } else {
        collect_backends!([KaniBackend, FluxBackend, ProptestBackend])
    };

    run_backends(
        &mut backend_outputs,
        &input_fn,
        &requires_exprs,
        &ensures_exprs,
        is_trusted,
    );

    let mut contract_attrs = Vec::new();
    let mut harnesses = Vec::new();

    for output in backend_outputs {
        if let Some(attrs) = output.attrs {
            contract_attrs.extend(attrs)
        }
        harnesses.push(output.harness);
    }

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

/// A marker that *cannot* be used alone.
/// If applied without `kedge_contracts::contract`,
/// this function will error.
#[proc_macro_attribute]
pub fn trusted(_args: TokenStream, _input_fn: TokenStream) -> TokenStream {
    quote! {
        compile_error!(
            "The `#[trusted]` attribute cannot be used alone. \n\
             You must add `#[kedge_contracts::contract]` to the function.\n\
             Example:\n   \
                 #[kedge_contracts::contract]\n   \
                 #[kedge_contracts::trusted]\n   \
                 #[kedge_contracts::ensures(x > 0)]\n   \
                 fn my_func(x: i8) ..."
        );
    }
    .into()
}
