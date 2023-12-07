#![allow(unused, dead_code)]

const LEN: usize = 2;

pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut stack = Vec::<i32>::with_capacity(LEN);
    for num in nums {
        match stack.partition_point(|e| *e < num) {
            LEN => return true,
            i if i == stack.len() => stack.push(num),
            i => stack[i] = num,
        }
    }
    false
}

pub fn increasing_triplet2(nums: Vec<i32>) -> bool {
    nums.into_iter()
        .scan(Vec::<i32>::with_capacity(LEN), |stack, num| {
            match stack.partition_point(|e| *e < num) {
                LEN => Some(true),
                i if i == stack.len() => {
                    stack.push(num);
                    Some(false)
                }
                i => {
                    stack[i] = num;
                    Some(false)
                }
            }
        })
        .any(|found| found)
}

pub fn increasing_triplet3(nums: Vec<i32>) -> bool {
    let (mut first, mut second) = (i32::MAX, i32::MAX);
    for ele in nums {
        if ele < first {
            first = ele;
        } else if ele < second {
            second = ele
        } else {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let result = increasing_triplet(vec![1, 2, 3, 4, 5]);
        assert!(result);
    }

    #[test]
    fn ex2() {
        let result = increasing_triplet(vec![5, 4, 3, 2, 1]);
        assert!(!result)
    }

    #[test]
    fn ex3() {
        let result = increasing_triplet(vec![2, 1, 5, 0, 4, 6]);
        assert!(result)
    }

    #[test]
    fn ex4() {
        let result = increasing_triplet(vec![20, 100, 10, 12, 5, 13]);
        assert!(result)
    }
}
