pub fn bon_appetit(bill: &[i32], k: i32, b: i32) {
    let k_usize = k as usize;
    
    // Рахуємо суму всього, крім страви, яку не їла Анна (індекс k)
    let total_to_split: i32 = bill.iter()
        .enumerate()
        .filter(|&(i, _)| i != k_usize)
        .map(|(_, &price)| price)
        .sum();
    
    let annas_actual_share = total_to_split / 2;

    if b == annas_actual_share {
        println!("Bon Appetit");
    } else {
        // Якщо Браян взяв більше, виводимо різницю
        println!("{}", b - annas_actual_share);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bon_appetit_fair() {
        // Анна не їла страву за 10 (індекс 1), сума (3+2+9)=14, доля 7. Браян просить 7.
        bon_appetit(&[3, 10, 2, 9], 1, 7); 
    }
}