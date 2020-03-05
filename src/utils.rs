use std::cmp::max;

pub trait Reversable {
    fn reverse(&self) -> Self;
}

impl Reversable for usize {
    fn reverse(&self) -> Self {
        let mut val: usize = *self;
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
    
    for i in 0..cols {
        for j in 0..(rows - window) {
            let mut item = 1;
            for k in 0..window {
                item *= nums[i][j + k];
            }
            ans = max(ans, item);
        }
    }

    for i in 0..(cols-window) {
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
    let mut ans = 0;
    for d in 1..((n as f64).sqrt() as usize) {
        if n % d == 0 {
            ans += 1;
            if n / d != d {
                ans += 1;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverses_numbers() {
        assert_eq!(123, 321usize.reverse());
    }
}