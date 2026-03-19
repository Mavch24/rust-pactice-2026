pub fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut count = 0;
    let m_usize = m as usize;

    // Якщо плитка коротша, ніж потрібний шматочок, варіантів 0
    if s.len() < m_usize {
        return 0;
    }

    // Проходимо по плитці вікном розміром m
    for i in 0..=(s.len() - m_usize) {
        let sum: i32 = s[i..(i + m_usize)].iter().sum();
        if sum == d {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birthday() {
        assert_eq!(birthday(&[1, 2, 1, 3, 2], 3, 2), 2);
        assert_eq!(birthday(&[4], 4, 1), 1);
    }
}