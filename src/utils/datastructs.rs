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
        if self.curr == self.end {
            None
        } else if self.curr > self.end {
            self.curr -= 1;
            Some(self.curr)
        } else {
            self.curr += 1;
            Some(self.curr)
        }
    }
}
