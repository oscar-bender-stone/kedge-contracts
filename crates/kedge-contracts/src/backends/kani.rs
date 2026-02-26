// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

pub fn requires(conditions: TokenStream, input_fn: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input_fn as ItemFn);

    // Keep conditions as they are
    // TODO: provide light validation
    // depending on backend?
    let conditions = proc_macro2::TokenStream::from(conditions);

    quote! {
        #[cfg_attr(kani, kani::requires(#conditions))]
        #input_fn
    }
    .into()
}

// TODO: share relevant code with ensures,
// including validation
pub fn ensures(conditions: TokenStream, input_fn: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input_fn as ItemFn);

    // Keep conditions as they are
    // TODO: provide light validation
    // depending on backend?
    let conditions = proc_macro2::TokenStream::from(conditions);

    quote! {
        #[cfg_attr(kani, kani::ensures(#conditions))]
        #input_fn
    }
    .into()
}
