#![allow(unused)]

pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut k = k;
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
        end += 1;
    }
    (end - start) as i32
}

pub fn longest_ones2(nums: Vec<i32>, k: i32) -> i32 {
    let mut longest: i32 = 0;
    let mut left: usize = 0;
    let mut z_cnt = 0;

    for right in 0..nums.len() {
        z_cnt += (nums[right] == 0) as i32;

        while z_cnt > k {
            z_cnt -= (nums[left] == 0) as i32;
            left += 1;
        }
        longest = std::cmp::max(longest, (right - left + 1) as i32);
    }
    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        assert_eq!(longest_ones(input, 2), 6);
    }

    #[test]
    fn ex2() {
        let input = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        assert_eq!(longest_ones(input, 3), 10);
    }
}
