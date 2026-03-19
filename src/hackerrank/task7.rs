pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;

    // Шукаємо числа від максимального в A до мінімального в B
    let start = *a.iter().max().unwrap();
    let end = *b.iter().min().unwrap();

    for x in start..=end {
        // Перевірка 1: чи всі елементи A є дільниками x
        let is_a_factors = a.iter().all(|&factor| x % factor == 0);
        
        // Перевірка 2: чи x є дільником для всіх елементів B
        let is_x_factor_for_b = b.iter().all(|&val| val % x == 0);

        if is_a_factors && is_x_factor_for_b {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        assert_eq!(get_total_x(&[2, 4], &[16, 32, 96]), 3);
    }
}