#![allow(unused)]

use std::collections::{HashMap, HashSet};

pub struct FreqMap {
    map: [u32; 27], // 27 chars in alphabet
}

impl FreqMap {
    pub fn from_string(word: String) -> Self {
        let mut map = [0; 27];

        for c in word.chars() {
            let c_idx = c as usize - 'a' as usize;
            map[c_idx] += 1;
        }

        Self { map }
    }

    pub fn bitmask(&self) -> u32 {
        let mut val = 0u32;
        for e in self.map {
            if e > 0 {
                val |= 1;
            }
            val <<= 1;
        }
        val
    }

    pub fn get(&self) -> Vec<u32> {
        let mut freq = self.map.to_vec();
        freq.sort();
        freq
    }
}

struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let freq1 = FreqMap::from_string(word1);
        let freq2 = FreqMap::from_string(word2);

        if freq1.bitmask() != freq2.bitmask() {
            return false;
        }
        freq1.get() == freq2.get()
    }
}

pub fn nice(word1: String, word2: String) -> bool {
    let (mut map1, mut map2) = ([0; 26], [0; 26]);
    for c in word1.chars() {
        map1[c as usize - 0x61] += 1;
    }
    for c in word2.chars() {
        map2[c as usize - 0x61] += 1;
    }

    for i in 0..26 {
        if (map1[i] == 0 && map2[i] != 0) || (map1[i] != 0 && map2[i] == 0) {
            return false;
        }
    }

    map1.sort();
    map2.sort();

    map1 == map2
}

pub fn close_strings(word1: String, word2: String) -> bool {
    // let mut memo1 = vec![0;26];
    // let mut memo2 = vec![0;26];
    //
    // for c in word1.chars() {
    //     memo1[(c as usize - 'a' as usize)] += 1;
    // }

    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    for ele in word1.chars() {
        *map1.entry(ele).or_insert(1) += 1;
    }
    for ele in word2.chars() {
        *map2.entry(ele).or_insert(1) += 1;
    }

    let mut c1 = map1.values().collect::<Vec<_>>();
    let mut c2 = map2.values().collect::<Vec<_>>();
    c1.sort();
    c2.sort();

    map1.keys().collect::<HashSet<_>>() == map2.keys().collect::<HashSet<_>>() && c1 == c2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = ("abc", "bca");
        assert!(close_strings(input.0.into(), input.1.into()))
    }
    #[test]
    fn ex2() {
        let input = ("a", "aa");
        assert!(!close_strings(input.0.into(), input.1.into()))
    }
    #[test]
    fn ex3() {
        let input = ("cabbba", "abbccc");
        assert!(close_strings(input.0.into(), input.1.into()))
    }
    #[test]
    fn ex4() {
        let input = ("aaabbbbccddeeeeefffff", "aaaaabbcccdddeeeeffff");
        assert!(!close_strings(input.0.into(), input.1.into()))
    }
}
