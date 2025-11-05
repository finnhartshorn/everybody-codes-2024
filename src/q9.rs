#![allow(unused)]

use itertools::Itertools;

fn part1(data: &str) -> Option<u32> {
    let stamps = vec![1, 3, 5, 10];

    let brightness_levels: Vec<usize> = data.lines().map(|b| b.parse::<usize>().unwrap()).collect();
    let max_brightness = brightness_levels.iter().max().unwrap();
    let memo = calc_memo(max_brightness, &stamps);

    Some(brightness_levels.iter().map(|b| {
        memo[*b]
    }).sum())
}


fn part2(data: &str) -> Option<u32> {
    let stamps = vec![1, 3, 5, 10, 15, 16, 20, 24, 25, 30];

    let brightness_levels: Vec<usize> = data.lines().map(|b| b.parse::<usize>().unwrap()).collect();
    let max_brightness = brightness_levels.iter().max().unwrap();
    let memo = calc_memo(max_brightness, &stamps);

    Some(brightness_levels.iter().map(|b| {
        memo[*b]
    }).sum())
}


pub fn part3(data: &str) -> Option<u32> {
    let stamps: Vec<usize> = vec![1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101];

    let brightness_levels: Vec<usize> = data.lines().map(|b| b.parse::<usize>().unwrap()).collect();
    let max_brightness = brightness_levels.iter().max().unwrap();
    let memo = calc_memo(max_brightness, &stamps);

    Some(brightness_levels.iter().map(|b| {
        (b/2..b/2+50).into_iter().map(|i| {
            memo[i] + memo[b-i]
        }).min().unwrap()
    }).sum())
}

fn calc_memo(max: &usize, stamps: &[usize]) -> Vec<u32> {
    let mut memo = vec![0; max+1];
    for i in 1..max+1 {
        let min_coins = stamps.iter().filter(|s| **s <= i).map(|s| memo[i - s] + 1).min().unwrap();
        memo[i] = min_coins;
    }
    memo
}

#[cfg(test)]
mod tests {
    use core::assert_eq;

    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("2
4
7
16");
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part1_release() {
        let result = part1(include_str!("../data/everybody_codes_e2024_q09_p1.txt"));
        println!("{}", result.unwrap());
        assert_eq!(result, Some(13283));
    }

    #[test]
    fn test_part2() {
        let result = part2("33
41
55
99");
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part2_release() {
        let result = part2(include_str!("../data/everybody_codes_e2024_q09_p2.txt"));
        println!("{}", result.unwrap());
        assert_eq!(result, Some(5024));
    }


    #[test]
    fn test_part3() {
        let result = part3("156488
352486
546212");
        assert_eq!(result, Some(10449));
    }

    #[test]
    fn test_part3_release() {
        let result = part3(include_str!("../data/everybody_codes_e2024_q09_p3.txt"));
        println!("{}", result.unwrap());
        assert_eq!(result, Some(152038));
    }
}
