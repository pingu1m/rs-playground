pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    let mut i = 0;
    while i < asteroids.len() {
        let x = asteroids[i];
        if stack.is_empty() {
            stack.push(x);
        } else {
            let last_el = stack.pop().unwrap();
            if (last_el >= 0 && x >= 0) || (last_el < 0 && x < 0) || (last_el < x) {
                stack.push(last_el);
                stack.push(x);
            } else if last_el.abs() > x.abs() {
                stack.push(last_el);
            } else if last_el.abs() < x.abs() {
                continue;
            };
        };
        i += 1;
    }
    stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![5, 10, -5];
        assert_eq!(asteroid_collision(input), vec![5, 10]);
    }

    #[test]
    fn ex2() {
        let input = vec![8, -8];
        assert_eq!(asteroid_collision(input), vec![]);
    }

    #[test]
    fn ex3() {
        let input = vec![10, 2, -5];
        assert_eq!(asteroid_collision(input), vec![10]);
    }

    #[test]
    fn ex4() {
        let input = vec![-2, -1, 1, 2];
        assert_eq!(asteroid_collision(input), vec![-2, -1, 1, 2]);
    }

    #[test]
    fn ex5() {
        let input = vec![-2, -2, -2, 1];
        assert_eq!(asteroid_collision(input), vec![-2, -2, -2, 1]);
    }
}
