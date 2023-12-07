use std::collections::HashMap;

pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let mut map = HashMap::<&[i32], i32>::new();
    for row in grid.iter() {
        *map.entry(row).or_insert(0) += 1;
    }
    let mut ans = 0;
    let mut temp_vec = vec![0; grid.len()];
    for j in 0..grid.len() {
        for i in 0..grid.len() {
            temp_vec[i] = grid[i][j];
        }
        if let Some(&el) = map.get(&temp_vec[..]) {
            ans += el;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
        assert_eq!(equal_pairs(input), 1)
    }

    #[test]
    fn ex2() {
        let input = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];
        assert_eq!(equal_pairs(input), 3)
    }

    #[test]
    fn ex3() {
        let input = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 4],
            vec![2, 4, 2, 2],
            vec![2, 5, 2, 2],
        ];
        assert_eq!(equal_pairs(input), 3)
    }
}
