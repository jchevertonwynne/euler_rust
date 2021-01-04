use std::cmp::max;
use std::collections::HashSet;
use std::fs;

use num::pow::pow;
use num::BigInt;

mod collatz;
mod fib;
mod primes;
mod triangle;
mod utils;

use collatz::Collatz;
use fib::Fib;
use primes::PrimeEndless;
use primes::PrimeFactorCount;
use primes::PrimeSieve;
use triangle::TriangularNumber;
use utils::factor_count;
use utils::max_product_window;
use utils::proper_divisors;
use utils::Reversable;

fn main() {
    println!("p001: {}", problem001());
    println!("p002: {}", problem002());
    println!("p003: {}", problem003());
    println!("p004: {}", problem004());
    println!("p005: {}", problem005());
    println!("p006: {}", problem006());
    println!("p007: {}", problem007());
    println!("p008: {}", problem008());
    println!("p009: {}", problem009());
    println!("p010: {}", problem010());
    println!("p011: {}", problem011());
    println!("p012: {}", problem012());
    println!("p013: {}", problem013());
    println!("p014: {}", problem014());
    println!("p015: {}", problem015());
    println!("p016: {}", problem016());
    // println!("p017: {}", problem017());
    println!("p018: {}", problem018());
    // println!("p019: {}", problem019());
    println!("p020: {}", problem020());
    println!("p021: {}", problem021());
    println!("p022: {}", problem022());
    println!("p023: {}", problem023());
    // println!("p024: {}", problem024());
    println!("p025: {}", problem025());
}

fn problem001() -> usize {
    (1..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

fn problem002() -> usize {
    Fib::new()
        .limit(4_000_000usize)
        .filter(|x| x % 2 == 0)
        .sum()
}

fn problem003() -> usize {
    let num = 600_851_475_143;
    let limit = (num as f64).sqrt() as usize + 1;
    PrimeSieve::new(limit)
        .filter(|p| num % p == 0)
        .last()
        .unwrap()
}

fn problem004() -> usize {
    (100..1000)
        .flat_map(|i| (i..1000).map(move |j| i * j).filter(|&v| v == v.reverse()))
        .max()
        .unwrap()
}

fn problem005() -> usize {
    (1..=20)
        .map(PrimeFactorCount::new)
        .fold(PrimeFactorCount::empty(), |acc, val| {
            acc.minimal_combine(val)
        })
        .to_num()
}

fn problem006() -> usize {
    let square_sum = (1..101).sum::<usize>().pow(2);
    let sum_square = (1..101).map(|n| n * n).sum::<usize>();
    square_sum - sum_square
}

fn problem007() -> usize {
    let mut primes = PrimeEndless::new().pre_calc(PrimeSieve::new(105_000).collect());
    primes.nth(10_000).unwrap()
}

fn problem008() -> usize {
    let nums = fs::read_to_string("files/problem008.txt").unwrap();
    let nums: Vec<usize> = nums
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    (0..(1000 - 13))
        .map(|i| nums[i..(i + 13)].iter().product())
        .max()
        .unwrap()
}

fn problem009() -> usize {
    (1usize..1000)
        .flat_map(|a| {
            ((a + 1)..(1000 - a)).filter_map(move |b| {
                let c = 1000 - a - b;
                match a != c && b != c && a.pow(2) + b.pow(2) == c.pow(2) {
                    true => Some(a * b * c),
                    false => None,
                }
            })
        })
        .next()
        .unwrap()
}

fn problem010() -> usize {
    PrimeSieve::new(2_000_000).sum()
}

fn problem011() -> usize {
    let nums = fs::read_to_string("files/problem011.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    max_product_window(nums, 4)
}

fn problem012() -> usize {
    TriangularNumber::new()
        .find(|&t| factor_count(t) > 500)
        .unwrap()
}

fn problem013() -> usize {
    let sum: BigInt = fs::read_to_string("files/problem013.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<BigInt>().unwrap())
        .sum();
    sum.to_string()[..10].to_string().parse().unwrap()
}

fn problem014() -> u128 {
    let mut c = Collatz::new();
    let (num, _) = (1..1_000_000).map(|i| (i, c.collatz(i))).fold(
        (0, 0),
        |(acc_num, acc_dist), (f_num, f_dist)| match f_dist > acc_dist {
            true => (f_num, f_dist),
            false => (acc_num, acc_dist),
        },
    );
    num
}

fn problem015() -> usize {
    let top = PrimeFactorCount::factorial(40);
    let b1 = PrimeFactorCount::factorial(20);
    let b2 = PrimeFactorCount::factorial(20);
    (top / (b1 * b2)).to_num()
}

fn problem016() -> usize {
    let big = pow(BigInt::from(2), 1000).to_string();
    big.chars().map(|c| c.to_digit(10).unwrap() as usize).sum()
}

fn problem018() -> usize {
    let mut triangle: Vec<Vec<usize>> = fs::read_to_string("files/problem018.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    while triangle.len() > 1 {
        let top = triangle.pop().unwrap();
        let best: Vec<usize> = top
            .iter()
            .skip(1)
            .zip(top.iter())
            .map(|(&i, &j)| max(i, j))
            .collect();
        let next = triangle.pop().unwrap();
        let added = next.iter().zip(best.iter()).map(|(&a, &b)| a + b).collect();
        triangle.push(added);
    }
    triangle[0][0]
}

fn problem020() -> usize {
    (1..=100)
        .map(BigInt::from)
        .fold(BigInt::from(1), |acc, v| acc * v)
        .to_string()
        .chars()
        .fold(0, |acc, v| acc + v.to_digit(10).unwrap() as usize)
}

fn problem021() -> usize {
    (1..10_000)
        .filter(|&i| {
            let div_sum = proper_divisors(i).iter().sum();
            match div_sum < 10_000 && i != div_sum {
                true => proper_divisors(div_sum).iter().sum::<usize>() == i,
                false => false,
            }
        })
        .sum()
}

fn problem022() -> usize {
    let file = fs::read_to_string("files/problem022.txt").unwrap();
    let file = file.replace("\"", "");
    let mut names: Vec<&str> = file.split(',').collect();
    names.sort_unstable();
    names
        .iter()
        .zip(1..=names.len())
        .map(|(name, i)| name.chars().map(|c| c as usize - 64).sum::<usize>() * i)
        .sum()
}

fn problem023() -> usize {
    let abundants: HashSet<usize> = (1..=28_123)
        .filter(|&i| proper_divisors(i).iter().sum::<usize>() > i)
        .collect();
    (1..=28_123)
        .filter(|&i| {
            !abundants
                .iter()
                .any(|&j| i > j && abundants.contains(&(i - j)))
        })
        .sum()
}

fn problem025() -> usize {
    Fib::<BigInt>::new()
        .map(|v| v.to_string().len())
        .enumerate()
        .find(|(_, v)| *v == 1000)
        .unwrap()
        .0
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problems_test_010() {
        assert_eq!(problem001(), 233_168);
        assert_eq!(problem002(), 4_613_732);
        assert_eq!(problem003(), 6_857);
        assert_eq!(problem004(), 906_609);
        assert_eq!(problem005(), 232_792_560);
        assert_eq!(problem006(), 25_164_150);
        assert_eq!(problem007(), 104_743);
        assert_eq!(problem008(), 23_514_624_000);
        assert_eq!(problem009(), 31_875_000);
        assert_eq!(problem010(), 142_913_828_922);
    }

    #[test]
    fn problems_test_020() {
        assert_eq!(problem011(), 70_600_674);
        assert_eq!(problem012(), 76_576_500);
        assert_eq!(problem013(), 5_537_376_230);
        assert_eq!(problem014(), 837_799);
        assert_eq!(problem015(), 137_846_528_820);
        assert_eq!(problem016(), 1_366);
        // assert_eq!(problem017(), 906609);
        assert_eq!(problem018(), 1_074);
        // assert_eq!(problem019(), 906609);
        assert_eq!(problem020(), 648);
    }

    #[test]
    fn problems_test_030() {
        assert_eq!(problem021(), 31_626);
        assert_eq!(problem022(), 871_198_282);
        assert_eq!(problem023(), 4_179_871);
        // assert_eq!(problem024(), 648);
        assert_eq!(problem025(), 4_782);
    }
}
