pub fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    let n_usize = n as usize;

    for i in 0..n_usize {
        for j in (i + 1)..n_usize {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisible_sum_pairs() {
        assert_eq!(divisible_sum_pairs(6, 3, &[1, 3, 2, 6, 1, 2]), 5);
    }
}