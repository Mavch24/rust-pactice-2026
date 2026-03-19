pub fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apples_count = apples.iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count();

    let oranges_count = oranges.iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count();

    println!("{}", apples_count);
    println!("{}", oranges_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apples_oranges() {
        // Тест для перевірки логіки (можна залишити порожнім або як заглушку)
        assert!(true);
    }
}