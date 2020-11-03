use std::{collections::HashSet, cmp::max};
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
                let mut sieve: Vec<bool> = vec![true; limit+1];
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
pub struct PrimeFactorCount {
    pub factors: HashMap<usize, usize>,
}

impl PrimeFactorCount {
    pub fn new(num: usize) -> PrimeFactorCount {
        let primes: Vec<usize> = PrimeSieve::new(num).collect();
        let mut factors: HashMap<usize, usize> = HashMap::new();
        let mut n = num;

        while n > 1 {
            if let Some(&prime) = primes.iter().find(|&&p| n % p == 0) {
                match &factors.get(&prime) {
                    Some(&amount) => factors.insert(prime, amount + 1),
                    None => factors.insert(prime, 1),
                };
                n /= prime;
            }
        }

        PrimeFactorCount {
            factors
        }
    }

    pub fn empty() -> PrimeFactorCount {
        PrimeFactorCount {
            factors: HashMap::new(),
        }
    }

    pub fn factorial(num: usize) -> PrimeFactorCount {
        (1..=num)
            .map(PrimeFactorCount::new)
            .fold(PrimeFactorCount::empty(), |acc, new| acc *  new)
    }

    pub fn to_num(&self) -> usize {
        match self.factors.len() {
            0 => 0,
            _ => self.factors.iter()
                .fold(1, |acc, (key, val)| acc * key.pow(*val as u32))
        }
    }

    pub fn minimal_combine(&self, other: Self) -> PrimeFactorCount {
        PrimeFactorCount {
            factors: self.factors.iter()
                .chain(other.factors.iter())
                .map(|v| *v.0)
                .collect::<HashSet<usize>>()
                .iter()
                .map(|&v| {
                    match (self.factors.get(&v), other.factors.get(&v)){
                        (Some(&a), Some(&b)) => (v, max(a, b)),
                        (Some(&a), None) => (v, a),
                        (None, Some(&b)) => (v, b),
                        _ => panic!("how did this happen")
                    }
                }).collect()
        }
    }
}

impl Mul for PrimeFactorCount {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut factors = HashMap::new();
        for (key, val) in self.factors {
            factors.insert(key, val);
        }
        for (key, val) in rhs.factors {
            match &factors.get(&key) {
                Some(&pre_val) => factors.insert(key, val + pre_val),
                None => factors.insert(key, val),
            };
        }
        PrimeFactorCount {
            factors,
        }
    }
}

impl Add for PrimeFactorCount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let own_val = self.to_num();
        let other_val = other.to_num();
        PrimeFactorCount::new(own_val + other_val)
    }
}

impl Div for PrimeFactorCount {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut factors = HashMap::new();
        for (key, val) in self.factors {
            factors.insert(key, val);
        }
        for (key, val) in rhs.factors {
            match &factors.get(&key) {
                Some(&pre_val) => { 
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
        
        PrimeFactorCount {
            factors,
        }
    }
}

impl PartialEq for PrimeFactorCount {
    fn eq(&self, other: &Self) -> bool {
        self.factors.len() == other.factors.len() && 
            self.factors.iter()
                .all(|(key, val)| other.factors.get(key) == Some(val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_prime_factor_set() {
        let pf = PrimeFactorCount::new(200);
        assert_eq!(pf.factors.len(), 2);
        assert_eq!(pf.factors.values().sum::<usize>(), 5);
        assert_eq!(*pf.factors.get(&2).unwrap(), 3);
        assert_eq!(*pf.factors.get(&5).unwrap(), 2);
    }

    #[test]
    fn should_be_able_to_check_equality() {
        let p1 = PrimeFactorCount::new(200);
        let p2 = PrimeFactorCount::new(200);
        assert_eq!(p1, p2);
    }

    #[test]
    fn should_be_able_to_multiply() {
        let p1 = PrimeFactorCount::new(200);
        let p2 = PrimeFactorCount::new(4);
        let combined = p1 * p2;
        let expected = PrimeFactorCount::new(800);
        assert_eq!(combined, expected);
    }

    #[test]
    fn should_be_able_to_convert_back() {
        let expected = 200;
        let pf = PrimeFactorCount::new(expected);
        assert_eq!(pf.to_num(), expected);

        let expected = 0;
        let pf = PrimeFactorCount::new(expected);
        assert_eq!(pf.to_num(), expected);
    }

    #[test]
    fn should_be_able_to_add() {
        let p1 = PrimeFactorCount::new(150);
        let p2 = PrimeFactorCount::new(50);
        let combined = p1 + p2;
        let expected = PrimeFactorCount::new(200);
        assert_eq!(combined, expected);
    }

    #[test]
    fn should_be_able_to_divide() {
        let p1 = PrimeFactorCount::new(200);
        let p2 = PrimeFactorCount::new(100);
        let combined = p1 / p2;
        let expected = PrimeFactorCount::new(2);
        assert_eq!(combined, expected);
    }
}