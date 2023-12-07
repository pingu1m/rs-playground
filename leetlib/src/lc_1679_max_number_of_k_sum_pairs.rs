use std::collections::HashMap;

pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;

    for ele in nums {
        let delta = k - ele;
        if let Some(curr) = map.get_mut(&delta) {
            if *curr > 0 {
                result += 1;
                *curr -= 1;
                continue;
            }
        }
        if let Some(curr) = map.get_mut(&ele) {
            *curr += 1;
        } else {
            map.insert(ele, 1);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![1, 2, 3, 4];
        assert_eq!(max_operations(input, 5), 2);
    }

    #[test]
    fn ex2() {
        let input = vec![3, 1, 3, 4, 3];
        assert_eq!(max_operations(input, 6), 1);
    }

    #[test]
    fn ex3() {
        let input = vec![2, 5, 4, 4, 1, 3, 4, 4, 1, 4, 4, 1, 2, 1, 2, 2, 3, 2, 4, 2];
        assert_eq!(max_operations(input, 3), 4)
    }
}
