// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

mod backends;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn contract(_args: TokenStream, input_fn: TokenStream) -> TokenStream {
    backends::kani::contract(_args, input_fn)
}

#[proc_macro_attribute]
pub fn requires(conditions: TokenStream, input_fn: TokenStream) -> TokenStream {
    backends::kani::requires(conditions, input_fn)
}

#[proc_macro_attribute]
pub fn ensures(conditions: TokenStream, input_fn: TokenStream) -> TokenStream {
    backends::kani::ensures(conditions, input_fn)
}
