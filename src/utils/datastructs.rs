use std::cmp::Ordering;

pub struct RevRange {
    curr: i32,
    end: i32,
}

impl RevRange {
    pub fn new(start: i32, end: i32) -> RevRange {
        RevRange { curr: start, end }
    }
}

impl Iterator for RevRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr.cmp(&self.end) {
            Ordering::Equal => None,
            Ordering::Greater => {
                self.curr -= 1;
                Some(self.curr)
            }
            Ordering::Less => {
                self.curr += 1;
                Some(self.curr)
            }
        }
    }
}
