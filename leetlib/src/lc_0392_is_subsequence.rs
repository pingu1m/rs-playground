pub fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }
    if t.is_empty() {
        return false;
    }
    let s: Vec<char> = s.chars().collect();
    let mut i = 0;
    for ele in t.chars() {
        if i >= (s.len()) {
            break;
        }
        if s[i] == ele {
            i += 1;
        }
    }
    i == s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        assert!(is_subsequence(s, t));
    }

    #[test]
    fn ex2() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        assert!(!is_subsequence(s, t));
    }
}
