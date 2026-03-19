pub fn getMoneySpent(keyboards: &[i32], drives: &[i32], b: i32) -> i32 {
    let mut max_spent = -1;

    for &k in keyboards {
        for &d in drives {
            let total = k + d;
            // Перевіряємо, чи вписуємось в бюджет і чи це більше за попередній результат
            if total <= b && total > max_spent {
                max_spent = total;
            }
        }
    }

    max_spent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electronics_shop() {
        assert_eq!(getMoneySpent(&[3, 1], &[5, 2, 8], 10), 9);
        assert_eq!(getMoneySpent(&[4], &[5], 5), -1);
    }
}