use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let nums1 = nums1.clone().into_iter().collect::<HashSet<i32>>();
    let nums2 = nums2.into_iter().collect::<HashSet<i32>>();
    vec![
        nums1.difference(&nums2).cloned().collect::<Vec<i32>>(),
        nums2.difference(&nums1).cloned().collect::<Vec<i32>>(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = (vec![1, 2, 3], vec![2, 4, 6]);
        assert_eq!(
            find_difference(input.0, input.1),
            vec![vec![1, 3], vec![4, 6]]
        );
    }

    #[test]
    fn ex2() {
        let input = (vec![1, 2, 3, 3], vec![1, 1, 2, 2]);
        assert_eq!(find_difference(input.0, input.1), vec![vec![3], vec![]]);
    }
}
