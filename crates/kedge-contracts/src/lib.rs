// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn requires(conditions: TokenStream, input_fn: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input_fn as ItemFn);

    // Keep conditions as they are
    // TODO: provide light validation
    // depending on backend?
    let conditions = proc_macro2::TokenStream::from(conditions);

    quote! {
        #[kani::requires(#conditions)]
        #input_fn
    }
    .into()
}
