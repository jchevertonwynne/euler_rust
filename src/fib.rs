pub struct Fib {
    a: usize,
    b: usize,
    fib_type: FibType
}

enum FibType {
    FibRegular,
    FibLimited(usize),
}

impl Fib {
    pub fn new() -> Fib {
        Fib { 
            a: 0, 
            b: 1, 
            fib_type: FibType::FibRegular ,
        }
    }

    pub fn limit(&mut self, limit: usize) -> Fib {
        Fib {
            a: self.a,
            b: self.b,
            fib_type: FibType::FibLimited(limit),
        }
    }
}

impl Iterator for Fib {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.a + self.b;
        self.a = self.b;
        self.b = c;
        match self.fib_type {
            FibType::FibRegular => Some (self.a),
            FibType::FibLimited(limit) => if self.a < limit { Some(self.a) } else { None }
        }
    }
}
