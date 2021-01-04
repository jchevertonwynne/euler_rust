use std::cmp::max;

pub trait Reversable {
    fn reverse(&self) -> Self;
}

impl Reversable for usize {
    fn reverse(&self) -> Self {
        let mut val = *self;
        let mut ans = 0;
        while val > 0 {
            let units = val % 10;
            ans *= 10;
            ans += units;
            val /= 10;
        }
        ans
    }
}

pub fn max_product_window(nums: Vec<Vec<usize>>, window: usize) -> usize {
    let mut ans = 0;
    let cols = nums.len();
    let rows = nums[0].len();

    for num in &nums {
        for j in 0..(rows - window) {
            let mut item = 1;
            for k in 0..window {
                item *= num[j + k];
            }
            ans = max(ans, item);
        }
    }

    for i in 0..(cols - window) {
        for j in 0..rows {
            let mut item = 1;
            for k in 0..window {
                item *= nums[i + k][j];
            }
            ans = max(ans, item);
        }
    }

    for i in 0..(cols - window) {
        for j in 0..(rows - window) {
            let mut item = 1;
            for k in 0..window {
                item *= nums[i + k][j + k]
            }
            ans = max(ans, item);
        }
    }

    for i in window..cols {
        for j in 0..(rows - window) {
            let mut item = 1;
            for k in 0..window {
                item *= nums[i - k][j + k]
            }
            ans = max(ans, item);
        }
    }

    ans
}

pub fn factor_count(n: usize) -> usize {
    (1..=((n as f64).sqrt() as usize))
        .filter(|&d| n % d == 0)
        .map(|d| if d * d == n { 1 } else { 2 })
        .sum()
}

pub fn proper_divisors(n: usize) -> Vec<usize> {
    let mut divisors: Vec<usize> = vec![];
    for d in 1..=((n as f64).sqrt() as usize) {
        if n % d == 0 {
            divisors.push(d);
            let other = n / d;
            if other != d && other != n {
                divisors.push(other);
            }
        }
    }
    divisors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverses_numbers() {
        assert_eq!(123, 321usize.reverse());
    }

    #[test]
    fn gets_proper_divisors() {
        assert_eq!(proper_divisors(10), vec![1, 2, 5]);

        let mut result = proper_divisors(220);
        result.sort();
        assert_eq!(result, vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110]);

        let mut result = proper_divisors(284);
        result.sort();
        assert_eq!(result, vec![1, 2, 4, 71, 142]);
    }
}
