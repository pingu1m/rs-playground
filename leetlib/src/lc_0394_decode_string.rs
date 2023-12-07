pub fn decode_string(s: String) -> String {
    let mut stack = vec![];
    for ele in s.chars() {
        if ele == ']' {
            let mut string_seg = String::new();
            loop {
                let sub_el = stack.pop().unwrap();

                if sub_el == '[' {
                    break;
                } else {
                    string_seg.push(sub_el);
                }
            }
            string_seg = string_seg.chars().rev().collect::<String>();
            let mut count_seg = vec![];
            loop {
                if !stack.is_empty() {
                    let sub_el = stack.pop().unwrap();
                    if "1234567890".contains(sub_el) {
                        count_seg.push(sub_el);
                    } else {
                        stack.push(sub_el);
                        break;
                    }
                } else {
                    break;
                }
            }
            let count_num = count_seg
                .iter()
                .enumerate()
                .map(|el| {
                    let (i, x) = el;
                    x.to_string().parse::<i32>().unwrap() * 10_i32.pow(i as u32)
                })
                .sum::<i32>();

            dbg!(stack.clone());
            dbg!(string_seg.clone());
            dbg!(count_num);
            dbg!(count_seg);
            stack.extend(string_seg.repeat(count_num as usize).chars())
        } else {
            stack.push(ele);
        }
    }
    // dbg!(stack.clone());

    stack.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "3[a]2[bc]".to_string();
        assert_eq!(decode_string(input), "aaabcbc".to_string());
    }

    #[test]
    fn ex2() {
        let input = "3[a2[c]]".to_string();
        assert_eq!(decode_string(input), "accaccacc".to_string());
    }

    #[test]
    fn ex3() {
        let input = "2[abc]3[cd]ef".to_string();
        assert_eq!(decode_string(input), "abcabccdcdcdef".to_string());
    }

    #[test]
    fn ex4() {
        let input = "100[leetcode]".to_string();
        assert_eq!(decode_string(input), "leetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcode".to_string());
    }
}
