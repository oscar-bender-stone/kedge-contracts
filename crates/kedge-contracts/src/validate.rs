// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use syn::Attribute;

/// Checks whether a given attribute
/// comes from kedge-contracts.
/// To avoid collisions with other crates,
/// require that the *full path* be used
pub fn is_kedge_attr(attr: &Attribute, attr_name: &str) -> bool {
    let path = attr.path();

    // Confirm there are exactly two segments,
    // and that the first matche `kedge_contracts`
    if path.segments.len() == 2 {
        let first_segment = &path.segments[0];
        let last_segment = &path.segments[1];

        return first_segment.ident == "kedge_contracts" && last_segment.ident == attr_name;
    }

    false
}
