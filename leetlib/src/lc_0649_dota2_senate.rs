use std::collections::VecDeque;

pub fn predict_party_victory(senate: String) -> String {
    let mut queue = senate.chars().collect::<VecDeque<_>>();
    let (mut r_count, mut d_count) = (0, 0);
    let (mut r_ban, mut d_ban) = (0, 0);
    for vote in &queue {
        if vote == &'R' {
            r_count += 1;
        } else {
            d_count += 1;
        }
    }
    if r_count == 0 {
        return "Dire".to_string();
    } else if d_count == 0 {
        return "Radiant".to_string();
    }
    while r_count > 0 && d_count > 0 {
        while let Some(vote) = queue.pop_front() {
            if vote == 'R' {
                if r_ban > 0 {
                    r_ban -= 1;
                    r_count -= 1;
                } else {
                    d_ban += 1;
                    queue.push_back(vote);
                }
            } else if vote == 'D' {
                if d_ban > 0 {
                    d_ban -= 1;
                    d_count -= 1;
                } else {
                    r_ban += 1;
                    queue.push_back(vote);
                }
            }
            if r_count == 0 {
                return "Dire".to_string();
            } else if d_count == 0 {
                return "Radiant".to_string();
            }
        }
    }
    if r_count == 0 {
        return "Radiant".to_string();
    };
    "Dire".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "RD".to_string();
        assert_eq!(predict_party_victory(input), "Radiant".to_string());
    }
    #[test]
    fn ex2() {
        let input = "RDD".to_string();
        assert_eq!(predict_party_victory(input), "Dire".to_string());
    }
}
