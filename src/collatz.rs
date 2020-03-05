use std::collections::HashMap;

pub struct Collatz {
    cache: HashMap::<u128, u128>,
}

impl Collatz {
    pub fn new() -> Collatz {
        Collatz {
            cache: HashMap::new(),
        }
    }

    pub fn collatz(&mut self, n: u128) -> u128 {
        match self.cache.get(&n) {
            Some(&val) => val,
            None => {
                let mut to_do = vec![n];
                loop {
                    if let Some(val) = to_do.pop() {
                        if val == 1 {
                            self.cache.insert(val, 1);
                        }
                        else {
                            let next = collatz(&val);
                            match self.cache.get(&next).copied() {
                                Some(dist) => { self.cache.insert(val, dist + 1); },
                                None => {
                                    to_do.push(val);
                                    to_do.push(next);
                                }
                            }
                        }
                    }
                    else {
                        break;
                    }
                }
                *self.cache.get(&n).unwrap()
            }
        }
    }
}

fn collatz(n: &u128) -> u128{
    if n % 2 == 0 {
        n / 2
    }
    else {
        3 * n + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_collatz_with_cache() {
        let mut c = Collatz::new();
        assert_eq!(c.cache.len(), 0);

        let expected = 10;
        let result = c.collatz(13);
        assert_eq!(expected, result);
        assert!(c.cache.len() > 0);

        let expected = 26;
        let result = c.collatz(100);
        assert_eq!(expected, result);
    }
}