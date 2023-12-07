#![allow(dead_code, unused)]

struct Solution;
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut temp: Vec<_> = s.chars().filter(|&c| "aeiouAEIOU".contains(c)).collect();
        s.chars()
            .map(|c| {
                if "aeiouAEIOU".contains(c) {
                    temp.pop().unwrap()
                } else {
                    c
                }
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let result = Solution::reverse_vowels("hello".into());
        assert_eq!(result, "holle".to_string());
    }
    #[test]
    fn ex2() {
        let result = Solution::reverse_vowels("leetcode".into());
        assert_eq!(result, "leotcede".to_string());
    }
}
