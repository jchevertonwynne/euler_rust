use std::cmp::max;
use std::collections::HashMap;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;

pub struct PrimeSieve {
    sieve: Vec<bool>,
    index: usize,
}


impl PrimeSieve {
    pub fn new(limit: usize) -> PrimeSieve {
        let sieve = match limit {
            0 => vec![],
            _ => {
                let mut sieve: Vec<bool> = (0..limit+1).map(|_| true).collect();
                    sieve[0] = false;
                    sieve[1] = false;
                    sieve
            }
        };
        PrimeSieve {
            sieve,
            index: 0,
        }
    }
}

impl Iterator for PrimeSieve {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.index += 1;
            if self.index >= self.sieve.len() {
                return None;
            }
            if self.sieve[self.index] {
                for j in ((self.index * 2)..self.sieve.len()).step_by(self.index) {
                    self.sieve[j] = false;
                }
                return Some(self.index);
            }
        }
    }
}

pub struct PrimeEndless {
    primes: Vec<usize>,
    returns: usize,
}

impl PrimeEndless {
    pub fn new() -> PrimeEndless {
        PrimeEndless {
            primes: vec![2, 3],
            returns: 0,
        }
    }

    pub fn pre_calc(&self, known: Vec<usize>) -> PrimeEndless {
        PrimeEndless {
            primes: known,
            returns: self.returns
        }
    }
}

impl Iterator for PrimeEndless {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.returns < self.primes.len() {
            let value = Some(self.primes[self.returns]);
            self.returns += 1;
            return value
        }
        let mut check = self.primes[self.primes.len() - 1];
        loop {
            check += 1;
            if self.primes.iter().all(|&p| check % p != 0) {
                self.primes.push(check);
                return Some(check);
            }
        }
    }
}

#[derive(Debug)]
pub struct PrimeSet {
    pub factors: HashMap<usize, usize>,
}

impl PrimeSet {
    pub fn new(num: usize) -> PrimeSet {
        let primes: Vec<usize> = PrimeSieve::new(num).collect();
        let mut factors: HashMap<usize, usize> = HashMap::new();
        let mut n = num;

        while n > 1 {
            match primes.iter().find(|&&p| n % p == 0) {
                Some(&prime) => {
                    match factors.get(&prime).copied() {
                        Some(amount) => factors.insert(prime, amount + 1),
                        None => factors.insert(prime, 1),
                    };
                    n /= prime;
                }
                _ => (),
            }
        }

        PrimeSet {
            factors
        }
    }

    pub fn factorial(num: usize) -> PrimeSet {
        (1..=num)
            .map(PrimeSet::new)
            .fold(PrimeSet::new(0), |acc, new| acc *  new)
    }

    pub fn to_num(&self) -> usize {
        if self.factors.len() == 0 {
            return 0;
        }
        let mut ans = 1;
        self.factors.iter().for_each(|(key, val)| ans *= key.pow(*val as u32));
        ans
    }

    pub fn minimal_combine(&self, other: PrimeSet) -> PrimeSet {
        let mut factors: HashMap<usize, usize> = HashMap::new();
        for (&key, &val) in &self.factors {
            factors.insert(key, val);
        }
        for (key, val) in other.factors {
            match factors.get(&key).copied() {
                Some(pre_val) => { 
                    factors.insert(key, max(val, pre_val)); 
                },
                None => {
                    factors.insert(key, val);
                },
            }
        }
        PrimeSet {
            factors
        }
    }
}

impl Mul for PrimeSet {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut factors = HashMap::new();
        for (key, val) in self.factors {
            factors.insert(key, val);
        }
        for (key, val) in rhs.factors {
            match factors.get(&key).copied() {
                Some(pre_val) => { 
                    factors.insert(key, val + pre_val); 
                },
                None => {
                    factors.insert(key, val);
                },
            }
        }
        PrimeSet {
            factors,
        }
    }
}

impl Add for PrimeSet {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let own_val = self.to_num();
        let other_val = other.to_num();
        PrimeSet::new(own_val + other_val)
    }
}

impl Div for PrimeSet {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut factors = HashMap::new();
        for (key, val) in self.factors {
            factors.insert(key, val);
        }
        for (key, val) in rhs.factors {
            match factors.get(&key).copied() {
                Some(pre_val) => { 
                    let new_val = pre_val - val;
                    factors.insert(key, new_val); 
                },
                None => {
                    panic!("Incompatible division");
                },
            }
        }

        let factors = factors.into_iter()
            .filter(|&(_, val)| val != 0)
            .collect();
        
        PrimeSet {
            factors,
        }
    }
}

impl PartialEq for PrimeSet {
    fn eq(&self, other: &Self) -> bool {
        if self.factors.len() != other.factors.len() {
            false
        }
        else {
            self.factors.iter()
                .all(|(key, val)| other.factors.get(key) == Some(val))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_prime_factor_set() {
        let pf = PrimeSet::new(200);
        assert_eq!(pf.factors.len(), 2);
        assert_eq!(pf.factors.values().sum::<usize>(), 5);
        assert_eq!(*pf.factors.get(&2).unwrap(), 3);
        assert_eq!(*pf.factors.get(&5).unwrap(), 2);
    }

    #[test]
    fn should_be_able_to_check_equality() {
        let p1 = PrimeSet::new(200);
        let p2 = PrimeSet::new(200);
        assert_eq!(p1, p2);
    }

    #[test]
    fn should_be_able_to_multiply() {
        let p1 = PrimeSet::new(200);
        let p2 = PrimeSet::new(4);
        let combined = p1 * p2;
        let expected = PrimeSet::new(800);
        assert_eq!(combined, expected);
    }

    #[test]
    fn should_be_able_to_convert_back() {
        let expected = 200;
        let pf = PrimeSet::new(expected);
        assert_eq!(pf.to_num(), expected);

        let expected = 0;
        let pf = PrimeSet::new(expected);
        assert_eq!(pf.to_num(), expected);
    }

    #[test]
    fn should_be_able_to_add() {
        let p1 = PrimeSet::new(150);
        let p2 = PrimeSet::new(50);
        let combined = p1 + p2;
        let expected = PrimeSet::new(200);
        assert_eq!(combined, expected);
    }

    #[test]
    fn should_be_able_to_divide() {
        let p1 = PrimeSet::new(200);
        let p2 = PrimeSet::new(100);
        let combined = p1 / p2;
        let expected = PrimeSet::new(2);
        assert_eq!(combined, expected);
    }
}