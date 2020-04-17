use std::cmp::max;
use std::collections::HashSet;
use std::fs;

use num::BigInt;
use num::pow::pow;

use chrono::{DateTime, NaiveDate};

mod collatz;
mod fib;
mod primes;
mod triangle;
mod utils;

use collatz::Collatz;
use fib::Fib;
use primes::PrimeEndless;
use primes::PrimeSet;
use primes::PrimeSieve;
use triangle::TriangularNumber;
use utils::proper_divisors;
use utils::factor_count;
use utils::max_product_window;
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
    (1..1000)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

fn problem002() -> usize {
    Fib::new()
        .limit(4_000_000)
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
    let mut ans = 0;
    for i in 100..1000 {
        for j in i..1000 {
            let k = i * j;
            if k == k.reverse() {
                ans = max(ans, k);
            }
        }
    }
    ans
}

fn problem005() -> usize {
    (1..=20)
        .map(PrimeSet::new)
        .fold(PrimeSet::empty(), |acc, val| acc.minimal_combine(val))
        .to_num()
}

fn problem006() -> usize {
    let square_sum = (1..101)
        .sum::<usize>()
        .pow(2);
    let sum_square = (1..101)
        .map(|n| n * n)
        .sum::<usize>();
    square_sum - sum_square
}

fn problem007() -> usize {
    let primes = PrimeEndless::new()
        .pre_calc(PrimeSieve::new(105_000).collect());
    primes.skip(10_000)
        .next()
        .unwrap()
}

fn problem008() -> usize {
    let nums = fs::read_to_string("files/problem008.txt").unwrap();
    let nums: Vec<usize> = nums.chars()
        .filter(|&c| c != '\n')
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    (0..(1000-13))
        .map(|i| nums[i..(i + 13)].iter()
            .fold(1, |acc, v| acc * v))
        .max()
        .unwrap()
}

fn problem009() -> usize {
    for a in 1usize..1000 {
        for b in (a + 1)..(1000 - a) {
            let c = 1000 - a - b;
            if a != c && b != c && a.pow(2) + b.pow(2) == c.pow(2) {
                return a * b * c
            }
        }
    }
    panic!("uh oh");
}

fn problem010() -> usize {
    PrimeSieve::new(2_000_000).sum()
}

fn problem011() -> usize {
    let nums = fs::read_to_string("files/problem011.txt")
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect())
        .collect();
    max_product_window(nums, 4)
}

fn problem012() -> usize {
    TriangularNumber::new()
        .find(|&t| factor_count(t) > 500)
        .unwrap()
}

fn problem013() -> String {
    let sum: BigInt = fs::read_to_string("files/problem013.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<BigInt>().unwrap())
        .sum();
    format!("{}", sum)[..10].to_string()
}

fn problem014() -> u128 {
    let mut c = Collatz::new();
    let (num, _) = (1..1_000_000)
        .map(|i| (i, c.collatz(i)))
        .fold((0, 0), |(acc_num, acc_dist), (f_num, f_dist)| if f_dist > acc_dist { (f_num, f_dist) } else { (acc_num, acc_dist) });
    num
}

fn problem015() -> usize {
    let top = PrimeSet::factorial(40);
    let b1 = PrimeSet::factorial(20);
    let b2 = PrimeSet::factorial(20);
    (top / (b1 * b2)).to_num()
}

fn problem016() -> usize {
    let big = pow(BigInt::from(2), 1000).to_string();
    big.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum()
}

fn problem018() -> usize {
    let mut triangle: Vec<Vec<usize>> = fs::read_to_string("files/problem018.txt")
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect())
        .collect();
    while triangle.len() >= 2 {
        let top = triangle.pop().unwrap();
        let best: Vec<usize> = (0usize..(top.len() - 1))
            .map(|i| max(top[i], top[i + 1]))
            .collect();
        let next = triangle.pop().unwrap();
        let added = next.iter()
            .zip(best.iter())
            .map(|(a, b)| a + b)
            .collect();
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
    let mut names: Vec<&str> = file.split(",").collect();
    names.sort();
    names.iter()
        .zip(1..=names.len())
        .map(|(name, i)| name.chars()
            .map(|c| c as usize - 64)
            .sum::<usize>() * i)
        .sum()
}

fn problem023() -> usize {
    let mut abundants: HashSet<usize> = HashSet::new();
    for i in 1..=28_123 {
        if proper_divisors(i).iter().sum::<usize>() > i {
            abundants.insert(i);
        }
    }
    let mut ans = 0;
    for i in 1..=28_123 {
        if !abundants.iter().any(|&a| i > a && abundants.contains(&(i - a))) {
            ans += i
        }
    }
    ans
}

fn problem025() -> usize {
    for f in Fib::new() {
        if (f as f64).log10() > 999f64 {
            return f
        }
    }
    panic!("don't do this");
}