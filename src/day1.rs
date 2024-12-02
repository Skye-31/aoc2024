// Came from https://github.com/dvtkrlbs/AoC/blob/main/aoc_2024/src/day1.rs
// I did the day 1 solution in the browser console, but deciding to go with rust for the rest - other files are mine

#![allow(dead_code)]
use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unzip();

    left.sort();
    right.sort();

    let total = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (left_v, right_v)| acc + (left_v - right_v).abs());

    total
}

// Optimized solution of hypex with hashmap to have better complexity in the calculation
#[aoc(day1, part2)]
fn alternate_part2_optimized(input: &str) -> i32 {
    let mut hashmap = HashMap::new();
    let left: Vec<i32> = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .map(|(a, b)| {
            hashmap.entry(b).and_modify(|v| *v += 1).or_insert(1);

            a
        })
        .collect();

    left.iter()
        .fold(0, |acc, key| acc + (key * hashmap.get(key).unwrap_or(&0)))
}
