use std::ops::Add;
use num_traits::Num;

pub struct Fib<T> {
    a: T,
    b: T,
    fib_type: FibType<T>
}

enum FibType<T> {
    FibRegular,
    FibLimited(T),
}

impl<T: Clone + Default + Num> Fib<T> {
    pub fn new() -> Fib<T> {
        let mut a: T = Default::default();
        let mut b: T = Default::default();
        a.set_zero();
        b.set_one();
        Fib { 
            a,
            b,
            fib_type: FibType::FibRegular ,
        }
    }

    pub fn limit(&mut self, limit: T) -> Fib<T> {
        Fib {
            a: self.a.clone(),
            b: self.b.clone(),
            fib_type: FibType::FibLimited(limit),
        }
    }
}

impl <T: Clone + PartialOrd + Add<Output = T>> Iterator for Fib<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.a.clone() + self.b.clone();
        self.a = self.b.clone();
        self.b = c;
        match &self.fib_type {
            FibType::FibRegular => Some (self.a.clone()),
            FibType::FibLimited(limit) => if self.a < *limit { Some(self.a.clone()) } else { None }
        }
    }
}
