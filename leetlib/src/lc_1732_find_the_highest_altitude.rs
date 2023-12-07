pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    gain.iter()
        .fold((0, 0), |acc, x| {
            let (mut max, mut altitude) = acc;
            altitude += x;
            max = max.max(altitude);
            (max, altitude)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![-5, 1, 5, 0, -7];
        assert_eq!(largest_altitude(input), 1);
    }

    #[test]
    fn ex2() {
        let input = vec![-4, -3, -2, -1, 4, 3, 2];
        assert_eq!(largest_altitude(input), 0);
    }
}
