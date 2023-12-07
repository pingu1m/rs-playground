pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut k = 1;
    let (mut start, mut end) = (0, 0);
    while end < nums.len() {
        if nums[end] == 0 {
            k -= 1;
        }
        if k < 0 {
            if nums[start] == 0 {
                k += 1;
            }
            start += 1;
        }
        end += 1
    }
    (end - start - 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![1, 1, 0, 1];
        assert_eq!(longest_subarray(input), 3);
    }

    #[test]
    fn ex2() {
        let input = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
        assert_eq!(longest_subarray(input), 5);
    }

    #[test]
    fn ex3() {
        let input = vec![1, 1, 1];
        assert_eq!(longest_subarray(input), 2);
    }
}
