pub fn remove_stars(s: String) -> String {
    // let mut letters = Vec::new();
    //
    // for c in s.chars() {
    //     if c == '*' {
    //         if !letters.is_empty() {
    //             letters.pop();
    //         }
    //     } else {
    //         letters.push(c)
    //     }
    // }
    // letters.iter().collect::<String>()
    //
    s.chars()
        .fold(vec![], |mut a, x| {
            if x == '*' {
                a.pop();
            } else {
                a.push(x);
            };
            a
        })
        .iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "leet**cod*e".to_string();
        assert_eq!(remove_stars(input), "lecoe".to_string());
    }
    #[test]
    fn ex2() {
        let input = "erase*****".to_string();
        assert_eq!(remove_stars(input), "".to_string());
    }
}
