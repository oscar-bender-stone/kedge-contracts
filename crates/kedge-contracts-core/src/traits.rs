// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::quote;
use syn::{Expr, ItemFn, Path};

#[derive(Clone)]
pub struct Stub {
    pub original: Path,
    pub replacement: Path,
}
#[allow(unused)]
pub struct BackendOutput {
    pub attrs: Option<Vec<proc_macro2::TokenStream>>,
    pub harness: Option<proc_macro2::TokenStream>,
}

#[allow(unused)]
impl BackendOutput {
    pub fn new(
        attrs: Option<Vec<proc_macro2::TokenStream>>,
        harness: Option<proc_macro2::TokenStream>,
    ) -> Self {
        BackendOutput { attrs, harness }
    }
}

#[allow(unused)]
pub trait Backend {
    /// Generates a specification,
    /// and optionally verification tests
    fn generate(
        input_fn: &ItemFn,
        requires_exprs: &[Expr],
        ensures_exprs: &[Expr],
        stubs: &[Stub],
        is_trusted: bool,
    ) -> BackendOutput;

    /// Recursively replaces occurrences of `old(expr)` with `#replacement_path(expr)`
    /// in the given token stream. This allows users to write `old(x)` naturally
    /// in their contracts, solving the "moved/consumed value" issue.
    fn replace_old_exprs(
        stream: proc_macro2::TokenStream,
        replacement_path: &syn::Path,
    ) -> proc_macro2::TokenStream {
        let mut out = TokenStream::new();
        let mut iter = stream.into_iter().peekable();

        while let Some(tt) = iter.next() {
            match tt {
                TokenTree::Ident(ref id) if id == "old" => {
                    // If we see `old`, check if the next token
                    //  is a parenthesis group `(...)`
                    if let Some(TokenTree::Group(g)) = iter.peek() {
                        if g.delimiter() == Delimiter::Parenthesis {
                            let inner = Self::replace_old_exprs(g.stream(), replacement_path);
                            let mut new_group = Group::new(g.delimiter(), inner);
                            new_group.set_span(g.span());

                            // Emit the dynamically provided path
                            out.extend(quote! { #replacement_path #new_group });
                            iter.next(); // Consume the group so we don't process it twice
                            continue;
                        }
                    }
                    out.extend(quote! { #tt });
                }
                TokenTree::Group(g) => {
                    // Recurse into any other groups (brackets, braces, etc.)
                    let inner = Self::replace_old_exprs(g.stream(), replacement_path);
                    let mut new_group = Group::new(g.delimiter(), inner);
                    new_group.set_span(g.span());
                    out.extend(Some(TokenTree::Group(new_group)));
                }
                _ => out.extend(Some(tt)), // Pass through everything else untouched
            }
        }

        return out;
    }
}
