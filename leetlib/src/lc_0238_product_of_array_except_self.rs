#![allow(unused)]

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut answer = Vec::<i32>::with_capacity(nums.len());
    answer.push(1);
    for i in 1..nums.len() {
        answer.push(answer[i - 1] * nums[i - 1]);
    }
    let mut r = 1;
    for i in (0..nums.len()).rev() {
        answer[i] *= r;
        r *= nums[i];
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = [1, 2, 3, 4];
        assert_eq!(product_except_self(input.into()), [24, 12, 8, 6].to_owned())
    }

    #[test]
    fn ex2() {
        let input = [-1, 1, 0, -3, 3];
        assert_eq!(
            product_except_self(input.into()),
            [0, 0, 9, 0, 0].to_owned()
        )
    }
}
