pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut left_sum = 0;

    for ele in nums.clone() {
        sum += ele;
    }

    for (i, ele) in nums.iter().enumerate() {
        if left_sum == (sum - left_sum - ele) {
            return i as i32;
        }
        left_sum += ele
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(pivot_index(input), 3);
    }

    #[test]
    fn ex2() {
        let input = vec![1, 2, 3];
        assert_eq!(pivot_index(input), -1);
    }

    #[test]
    fn ex3() {
        let input = vec![2, 1, -1];
        assert_eq!(pivot_index(input), 0);
    }
}
