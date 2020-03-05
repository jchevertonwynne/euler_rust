use std::cmp::max;
use std::fs;


use num::BigInt;
use num::pow::pow;


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

use utils::factor_count;
use utils::max_product_window;
use utils::Reversable;

fn main() {
    println!("p001: {}", problem1());
    println!("p002: {}", problem2());
    println!("p003: {}", problem3());
    println!("p004: {}", problem4());
    println!("p005: {}", problem5());
    println!("p006: {}", problem6());
    println!("p007: {}", problem7());
    println!("p008: {}", problem8());
    println!("p009: {}", problem9());
    println!("p010: {}", problem10());
    println!("p011: {}", problem11());
    println!("p012: {}", problem12());
    println!("p013: {}", problem13());
    println!("p014: {}", problem14());
    println!("p015: {}", problem15());
    println!("p016: {}", problem16());
    println!("p018: {}", problem18());
}

fn problem1() -> usize {
    (1..1000)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

fn problem2() -> usize {
    Fib::new()
        .limit(4_000_000)
        .filter(|x| x % 2 == 0)
        .sum()
}

fn problem3() -> usize {
    let num = 600_851_475_143;
    let limit = (num as f64).sqrt() as usize + 1;
    let prime_factors: Vec<usize> = PrimeSieve::new(limit)
        .filter(|p| num % p == 0)
        .collect();
    prime_factors[prime_factors.len() - 1]
}

fn problem4() -> usize {
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

fn problem5() -> usize {
    (1..=20)
        .map(PrimeSet::new)
        .fold(PrimeSet::new(0), |acc, val| acc.minimal_combine(val))
        .to_num()
}

fn problem6() -> usize {
    let sum_square = (1..101)
        .map(|n| n * n)
        .sum::<usize>();
    let square_sum = (1..101)
        .sum::<usize>()
        .pow(2);
    square_sum - sum_square
}

fn problem7() -> usize {
    let primes = PrimeEndless::new()
        .pre_calc(PrimeSieve::new(105_000).collect());
    primes.skip(10_000)
        .next()
        .unwrap()
}

fn problem8() -> usize {
    let nums = fs::read_to_string("files/problem8.txt").unwrap();
    let nums: Vec<usize> = nums.chars()
        .filter(|&c| c != '\n')
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    (0..(1000-13))
        .map(|i| nums[i..(i + 13)].iter().fold(1, |acc, v| acc * v))
        .max()
        .unwrap()
}

fn problem9() -> usize {
    for a in 1usize..1000 {
        for b in (a + 1)..(1000 - a) {
            let c = 1000 - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return a * b * c
            }
        }
    }
    0
}

fn problem10() -> usize {
    PrimeSieve::new(2_000_000).sum()
}

fn problem11() -> usize {
    let nums: Vec<Vec<usize>> = fs::read_to_string("files/problem11.txt")
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect())
        .collect();
    max_product_window(nums, 4)
}

fn problem12() -> usize {
    TriangularNumber::new()
        .find(|&t| factor_count(t) > 500)
        .unwrap()
}

fn problem13() -> String {
    let sum: BigInt = fs::read_to_string("files/problem13.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<BigInt>().unwrap())
        .sum();
    format!("{}", sum)[..10].to_string()
}

fn problem14() -> u128 {
    let mut c = Collatz::new();
    let (num, _) = (1..1_000_000)
        .map(|i| (i, c.collatz(i)))
        .fold((0, 0), |(acc_num, acc_dist), (f_num, f_dist)| if f_dist > acc_dist { (f_num, f_dist) } else { (acc_num, acc_dist) });
    num
}

fn problem15() -> usize {
    let top = PrimeSet::factorial(40);
    let b1 = PrimeSet::factorial(20);
    let b2 = PrimeSet::factorial(20);
    let b3 = b1 * b2;
    (top / b3).to_num()
}

fn problem16() -> usize {
    let big = pow(BigInt::from(2), 1000).to_string();
    big.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum()
}

fn problem18() -> usize {
    let mut triangle: Vec<Vec<usize>> = fs::read_to_string("files/problem18.txt")
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
        let added: Vec<usize> = next.iter()
            .zip(best.iter())
            .map(|(a, b)| a + b)
            .collect();
        triangle.push(added);
    }
    triangle[0][0]
}