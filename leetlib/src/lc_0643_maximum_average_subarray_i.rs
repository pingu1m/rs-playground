pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let mut max = f64::MIN;
    let mut window: Vec<i32> = Vec::with_capacity(k);
    for ele in nums {
        window.push(ele);
        if window.len() == k {
            let local_sum: i32 = window.iter().sum();
            let avg = local_sum as f64 / k as f64;
            max = max.max(avg);
            window.remove(0);
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![1, 12, -5, -6, 50, 3];
        assert_eq!(find_max_average(input, 4), 12.75000);
    }

    #[test]
    fn ex2() {
        let input = vec![5];
        assert_eq!(find_max_average(input, 1), 5.00000)
    }
}
