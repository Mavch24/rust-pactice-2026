pub fn counting_valleys(_steps: i32, path: &str) -> i32 {
    let mut level = 0;
    let mut valleys = 0;

    for c in path.chars() {
        if c == 'U' {
            level += 1;
            // Якщо ми щойно піднялися на рівень 0, значить ми вийшли з долини
            if level == 0 {
                valleys += 1;
            }
        } else {
            level -= 1;
        }
    }
    valleys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valleys() {
        assert_eq!(counting_valleys(8, "UDDDUDUU"), 1);
    }
}