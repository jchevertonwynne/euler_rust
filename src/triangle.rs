pub struct TriangularNumber {
    tri: usize,
    counter: usize,
}

impl TriangularNumber {
    pub fn new() -> TriangularNumber {
        TriangularNumber {
            tri: 0,
            counter: 1,
        }
    }
}

impl Iterator for TriangularNumber {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.tri += self.counter;
        self.counter += 1;
        Some(self.tri)
    }
}
