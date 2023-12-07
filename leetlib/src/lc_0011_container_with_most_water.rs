pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut low, mut high) = (0, height.len() - 1);
    let mut max_area_value = i32::MIN;

    while low < high {
        let (low_val, high_val) = (height[low], height[high]);
        let height_val = low_val.min(high_val);
        max_area_value = max_area_value.max(height_val * (high - low) as i32);
        if low_val <= high_val {
            low += 1;
        } else {
            high -= 1;
        }
    }
    max_area_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(input), 49);
    }

    #[test]
    fn ex2() {
        let input = vec![1, 1];
        assert_eq!(max_area(input), 1);
    }
}
