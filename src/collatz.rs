use std::collections::HashMap;

pub struct Collatz {
    cache: HashMap<u128, u128>,
}

impl Collatz {
    pub fn new() -> Collatz {
        Collatz {
            cache: HashMap::new(),
        }
    }

    pub fn collatz(&mut self, n: u128) -> u128 {
        let mut to_do = vec![n];
        while !to_do.is_empty() {
            match to_do.pop() {
                Some(1) => {
                    self.cache.insert(1, 1);
                }
                Some(val) => {
                    let next = collatz(&val);
                    match &self.cache.get(&next) {
                        Some(&dist) => {
                            self.cache.insert(val, dist + 1);
                        }
                        None => {
                            to_do.push(val);
                            to_do.push(next);
                        }
                    }
                }
                _ => panic!("dont happen pls"),
            }
        }
        *self.cache.get(&n).unwrap()
    }
}

fn collatz(n: &u128) -> u128 {
    if n % 2 == 0 {
        n / 2
    } else {
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
