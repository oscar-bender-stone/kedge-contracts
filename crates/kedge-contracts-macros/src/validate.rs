// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use syn::Attribute;

/// Checks whether a given attribute matches a specific name
/// under authorized prefixes (kedge_contracts or kc).
pub fn is_kedge_attr(attr: &Attribute, attr_name: &str) -> bool {
    let path = attr.path();

    // We only care about paths with exactly two segments: prefix::name
    if path.segments.len() == 2 {
        let prefix = &path.segments[0].ident.to_string();
        let name = &path.segments[1].ident.to_string();

        let is_valid_prefix = prefix == "kedge_contracts" || prefix == "kc";
        let is_valid_name = name == attr_name;

        return is_valid_prefix && is_valid_name;
    }

    false
}
