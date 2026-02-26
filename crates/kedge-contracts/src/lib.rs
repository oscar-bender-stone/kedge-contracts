// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

mod backends;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn contract(_args: TokenStream, input_fn: TokenStream) -> TokenStream {
    backends::kani::contract(_args, input_fn)
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
            "The `#[requires]` attribute cannot be used alone. \n\
             You must add `#[kedge_contracts::contract]` to the function.\n\
             Example:\n   \
                 #[kedge_contracts::contract]\n   \
                 #[kedge_contracts::ensures(x > 0)]\n   \
                 fn my_func(x: i8) ..."
        );
    }
    .into()
}
