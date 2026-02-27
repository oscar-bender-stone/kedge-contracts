// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

#[cfg(test)]
mod tests {
    use kedge_contracts;

    #[kedge_contracts::contract]
    #[kedge_contracts::requires(x < 100)]
    #[kedge_contracts::ensures(*result > x)]
    fn my_test(x: i8) -> i8 {
        x + 1
    }
}
