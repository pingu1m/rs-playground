use std::collections::{HashMap, HashSet};

pub fn unique_occurences(arr: Vec<i32>) -> bool {
    let mut map = HashMap::new();
    for ele in arr.clone() {
        map.entry(ele).and_modify(|x| *x += 1).or_insert(1);
    }

    let arr_cmp = map.values().collect::<HashSet<_>>();
    arr_cmp.len() == map.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![1, 2, 2, 1, 1, 3];
        assert!(unique_occurences(input));
    }

    #[test]
    fn ex2() {
        let input = vec![1, 2];
        assert!(!unique_occurences(input));
    }

    #[test]
    fn ex3() {
        let input = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        assert!(unique_occurences(input));
    }
}
