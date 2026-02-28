// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use kedge_contracts as kc;

#[kc::contract]
#[kc::requires(x < 100)]
#[kc::ensures(*result > x)]
fn increment(x: i8) -> i8 {
    x + 1
}

#[kc::contract]
#[kc::requires(10 < x < 300)]
#[kc::ensures(*result > x)]
fn decrement(x: i16) -> i16 {
    x - 1
}
