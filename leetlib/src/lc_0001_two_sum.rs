#![allow(dead_code, unused)]

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];
        assert_eq!(two_sum(nums, target), expected);
    }
}
