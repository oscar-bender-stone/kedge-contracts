// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

#![allow(unused)]
extern crate flux_rs;

use super::traits::{Backend, BackendOutput};
use crate::validate::is_kedge_attr;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, FnArg, ItemFn, parse_macro_input};
use syn::{Pat, ReturnType};

struct FluxBackend;

impl Backend for FluxBackend {
    fn generate(
        input_fn: &ItemFn,
        requires_exprs: &[Expr],
        ensures_exprs: &[Expr],
    ) -> BackendOutput {
        let args = input_fn.sig.inputs.iter().map(|arg| match arg {
            FnArg::Typed(pat_type) => {
                let ty = &pat_type.ty;
                match &*pat_type.pat {
                    syn::Pat::Ident(p) => {
                        let ident = &p.ident;
                        quote! { #ident: #ty }
                    }
                    _ => quote! { _: #ty },
                }
            }
            FnArg::Receiver(r) => quote! { #r },
        });

        let return_type = match &input_fn.sig.output {
            ReturnType::Default => quote! { -> () },
            ReturnType::Type(_, ty) => quote! { -> #ty },
        };

        let requires_tokens = if requires_exprs.is_empty() {
            quote! {}
        } else {
            quote! { requires #(#requires_exprs)&&* }
        };

        let ensures_tokens = if ensures_exprs.is_empty() {
            quote! {}
        } else {
            quote! { ensures result: #(#ensures_exprs)&&* }
        };

        let flux_attr = quote! {
            #[cfg_attr(flux, flux_rs::spec(
                fn(#(#args),*) #return_type #requires_tokens #ensures_tokens
            ))]
        };

        BackendOutput::new(Some(vec![flux_attr]), None)
    }
}

// TODO: add support for structs and enums
// TODO: support more complex conditions
pub fn generate_flux_spec(
    input_fn: &ItemFn,
    requires: &[Expr],
    ensures: &[Expr],
) -> proc_macro2::TokenStream {
    let args = input_fn.sig.inputs.iter().map(|arg| match arg {
        FnArg::Typed(pat_type) => {
            let ty = &pat_type.ty;
            match &*pat_type.pat {
                syn::Pat::Ident(p) => {
                    let ident = &p.ident;
                    quote! { #ident: #ty }
                }
                _ => quote! { _: #ty },
            }
        }
        FnArg::Receiver(r) => quote! { #r },
    });

    let return_type = match &input_fn.sig.output {
        ReturnType::Default => quote! { -> () },
        ReturnType::Type(_, ty) => quote! { -> #ty },
    };

    let requires_tokens = if requires.is_empty() {
        quote! {}
    } else {
        quote! { requires #(#requires)&&* }
    };

    let ensures_tokens = if ensures.is_empty() {
        quote! {}
    } else {
        quote! { ensures result: #(#ensures)&&* }
    };

    quote! {
        #[cfg_attr(flux, flux_rs::spec(
            fn(#(#args),*) #return_type #requires_tokens #ensures_tokens
        ))]
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
    let flux_spec = generate_flux_spec(&input_fn, &requires_exprs, &ensures_exprs);

    quote! {
        #(#attrs)*
        #flux_spec
        #input_fn
    }
    .into()
}
