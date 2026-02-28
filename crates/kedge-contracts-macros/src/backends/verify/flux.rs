// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

#![allow(unused)]
extern crate flux_rs;

use crate::validate::is_kedge_attr;
use kedge_contracts_core::traits::{Backend, BackendOutput, Stub};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, FnArg, ItemFn, parse_macro_input};
use syn::{Pat, ReturnType};

pub(crate) struct FluxBackend;

impl Backend for FluxBackend {
    fn generate(
        input_fn: &ItemFn,
        requires_exprs: &[Expr],
        ensures_exprs: &[Expr],
        _stubs: &[Stub],
        is_trusted: bool,
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

        let flux_old_path: syn::Path = syn::parse_quote!(flux_rs::old);
        let requires_tokens = if ensures_exprs.is_empty() {
            quote! {}
        } else {
            let replaced_exprs: Vec<_> = requires_exprs
                .iter()
                .map(|expr| Self::replace_old_exprs(quote! { #expr }, &flux_old_path))
                .collect();
            quote! { requires #(#replaced_exprs)&&* }
        };

        let ensures_tokens = if ensures_exprs.is_empty() {
            quote! {}
        } else {
            let replaced_exprs: Vec<_> = ensures_exprs
                .iter()
                .map(|expr| Self::replace_old_exprs(quote! { #expr }, &flux_old_path))
                .collect();
            quote! { requires #(#replaced_exprs)&&* }
        };

        let spec_attr = quote! {
            #[cfg_attr(flux, flux_rs::spec(
                fn(#(#args),*) #return_type #requires_tokens #ensures_tokens
            ))]
        };

        let trusted_attr = if !is_trusted {
            quote! {}
        } else {
            quote! {#[cfg_attr(flux, flux_rs::trusted)]}
        };

        BackendOutput::new(Some(vec![spec_attr, trusted_attr]), None)
    }
}
