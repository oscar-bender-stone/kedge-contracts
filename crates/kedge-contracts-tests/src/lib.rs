// SPDX-FileCopyrightText: 2026 Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: MIT

use kedge_contracts;

#[cfg(test)]
mod tests {

    #[kedge_contracts::requires(x > 0)]
    #[kedge_contracts::ensures(x > 0)]
    fn my_test(x: int) -> int {
        x + 1
    }
}
