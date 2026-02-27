// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

#[cfg(any(kani, test))]
mod tests {
    use kedge_contracts;

    #[kedge_contracts::contract]
    #[kedge_contracts::requires(x < 100)]
    #[kedge_contracts::ensures(*result > x)]
    fn increment(x: i8) -> i8 {
        x + 1
    }

    #[kedge_contracts::contract]
    #[kedge_contracts::requires(10 < x < 300)]
    #[kedge_contracts::ensures(*result > x)]
    fn decrement(x: i16) -> i16 {
        x - 1
    }
}
