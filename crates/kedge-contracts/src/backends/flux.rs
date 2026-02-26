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

pub(crate) struct FluxBackend;

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
