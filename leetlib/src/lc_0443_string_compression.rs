#![allow(unused)]

pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut chars = chars;
    let mut result = String::new();
    let (mut i, mut j) = (0, 1);

    while i < j && j < chars.len() {
        if chars[i] == chars[j] {
            j += 1;
        } else {
            let temp = j - i;
            if temp > 1 {
                result.push_str(&format!("{}{}", chars[i], temp));
            } else {
                result.push(chars[i]);
            }
            i = j;
            j += 1;
        }
    }
    let temp = j - i;
    if temp > 1 {
        result.push_str(&format!("{}{}", chars[i], temp));
    } else {
        result.push(chars[i]);
    }
    // handle final edge case
    let mut answer: Vec<char> = result.chars().collect();
    std::mem::swap(chars, &mut answer);

    result.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut input = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(compress(input.as_mut()), 6);
    }

    #[test]
    fn ex2() {
        let mut input = vec!['a'];
        assert_eq!(compress(input.as_mut()), 1);
    }

    #[test]
    fn ex3() {
        let mut input = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        assert_eq!(compress(input.as_mut()), 4);
    }
}
