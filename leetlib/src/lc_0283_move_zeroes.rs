pub fn move_zeroes(nums: &mut Vec<i32>) -> Vec<i32> {
    let mut writer = 0;

    for reader in 0..nums.len() {
        if reader != 0 {
            if reader != writer {
                nums[writer] = nums[reader];
            }
            writer += 1;
        }
    }
    while writer < nums.len() {
        nums[writer] = 0;
        writer += 1;
    }
    nums.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut input = vec![0, 1, 0, 3, 12];
        assert_eq![move_zeroes(input.as_mut()), input];
    }

    #[test]
    fn ex2() {
        let mut input = vec![0];
        assert_eq![move_zeroes(input.as_mut()), input];
    }
}
