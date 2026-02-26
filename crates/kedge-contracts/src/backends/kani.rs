// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

pub fn contract(_args: TokenStream, input_fn: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(input_fn as ItemFn);
    let mut requires = Vec::new();
    let mut ensures = Vec::new();

    input_fn.attrs.retain(|attr| {
        let path = attr.path();
        ensures.push(attr.clone());

        if path.is_ident("requires") {
            requires.push(attr.clone());
            false
        } else if path.is_ident("ensures") {
            ensures.push(attr.clone());
            false
        } else {
            true
        }
    });

    quote! {
        #(#[cfg_attr(kani, kani::requires(#requires))])*
        #(#[cfg_attr(kani, kani::ensures(#requires))])*
        #input_fn
    }
    .into()
}

// Ignore args in requires and ensures;
// handled in contract
pub fn requires(_args: TokenStream, input_fn: TokenStream) -> TokenStream {
    input_fn
}

pub fn ensures(_args: TokenStream, input_fn: TokenStream) -> TokenStream {
    input_fn
}
