pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = vec![0; 6]; // Типи пташок від 1 до 5
    for &bird_type in arr {
        counts[bird_type as usize] += 1;
    }

    let mut max_count = 0;
    let mut result_bird = 1;

    for bird_type in 1..=5 {
        if counts[bird_type] > max_count {
            max_count = counts[bird_type];
            result_bird = bird_type as i32;
        }
    }
    result_bird
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds() {
        assert_eq!(migratory_birds(&[1, 4, 4, 4, 5, 3]), 4);
    }
}