pub fn page_count(n: i32, p: i32) -> i32 {
    // Кількість перегортань з початку
    let from_front = p / 2;
    
    // Кількість перегортань з кінця
    // (n/2) - загальна кількість розворотів у книзі
    let from_back = (n / 2) - (p / 2);
    
    if from_front < from_back {
        from_front
    } else {
        from_back
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_count() {
        assert_eq!(page_count(6, 2), 1);
        assert_eq!(page_count(5, 4), 0);
    }
}