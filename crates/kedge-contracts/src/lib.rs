// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

pub use kedge_contracts_macros::{contract, ensures, requires};

#[cfg(feature = "proptest")]
pub use proptest;
