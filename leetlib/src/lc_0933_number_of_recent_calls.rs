#![allow(unused)]

use std::collections::VecDeque;

struct RecentCounter {
    calls: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Self {
            calls: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while self.calls.front().map_or(false, |x| *x < t) {
            self.calls.pop_front();
        }
        self.calls.push_back(t + 3000);
        self.calls.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut rc = RecentCounter::new();
        let a1 = rc.ping(1); // requests = [1], range is [-2999,1], return 1
        let a2 = rc.ping(100); // requests = [1, 100], range is [-2900,100], return 2
        let a3 = rc.ping(3001); // requests = [1, 100, 3001], range is [1,3001], return 3
        let a4 = rc.ping(3002); // requests = [1, 100, 3001, 3002], range is [2,3002], return 3
        assert_eq!(a1, 1);
        assert_eq!(a2, 2);
        assert_eq!(a3, 3);
        assert_eq!(a4, 3);
    }
}
