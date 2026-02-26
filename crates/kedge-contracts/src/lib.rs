// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

mod backends;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn requires(conditions: TokenStream, input_fn: TokenStream) -> TokenStream {
    backends::kani::requires(conditions, input_fn)
}

// TODO: share relevant code with ensures,
// including validation
#[proc_macro_attribute]
pub fn ensures(conditions: TokenStream, input_fn: TokenStream) -> TokenStream {
    backends::kani::ensures(conditions, input_fn)
}
