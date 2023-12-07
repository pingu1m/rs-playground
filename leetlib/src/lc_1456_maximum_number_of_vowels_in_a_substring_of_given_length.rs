#![allow(unused)]

use std::collections::VecDeque;

pub fn max_vowels2(s: String, k: i32) -> i32 {
    s.chars()
        .collect::<Vec<char>>()
        .windows(k as usize)
        .map(|el| el.iter().filter(|&&c| "aeiou".contains(c)).count())
        .max()
        .unwrap() as i32
}
pub fn max_vowels3(s: String, k: i32) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let k = k as usize;
    let mut max = i32::MIN;
    let (mut slow, mut fast) = (0, 0);

    let mut vowel_count = 0;
    while fast < s.len() {
        let win_size = 1 + fast - slow;
        if "aeiou".contains(s[fast]) {
            vowel_count += 1;
        }
        if win_size == k {
            max = max.max(vowel_count);
            if "aeiou".contains(s[slow]) {
                vowel_count -= 1;
            }
            slow += 1;
        }
        fast += 1;
    }
    max
}

pub fn max_vowels(s: String, k: i32) -> i32 {
    let k = k as usize;

    let mut q = VecDeque::new();
    let mut count = 0;
    let mut max = i32::MIN;

    // match c {
    //     'a' | 'e' | 'i' | 'o' | 'u' => true,
    //     _ => false,
    // }
    fn is_vowel(c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
    }

    for c in s.chars() {
        if q.len() == k {
            let a = q.pop_front().unwrap();
            if is_vowel(a) {
                count -= 1;
            }
        }
        if is_vowel(c) {
            count += 1;
            max = max.max(count);
        }
        q.push_back(c);
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "abciiidef".to_string();
        assert_eq!(max_vowels(input, 3), 3);
    }

    #[test]
    fn ex2() {
        let input = "aeiou".to_string();
        assert_eq!(max_vowels(input, 2), 2);
    }

    #[test]
    fn ex3() {
        let input = "leetcode".to_string();
        assert_eq!(max_vowels(input, 3), 2);
    }
}
