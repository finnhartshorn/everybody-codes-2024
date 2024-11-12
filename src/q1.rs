#![allow(unused)]

use itertools::Itertools;

fn part1(data: &str) -> Option<u32> {
    Some(data
        .chars()
        .map(|c| match c {
            'B' => 1,
            'C' => 3,
            _ => 0,
        })
        .sum())
}

fn part2(data: &str) -> Option<u32> {
    Some(data.chars().chunks(2).into_iter().map(|c| {
        let cs = c.collect::<Vec<_>>();
        let result = match cs[0] {
                'B' => 1,
                'C' => 3,
                'D' => 5,
                _ => 0,
        } + match cs[1] {
                'B' => 1,
                'C' => 3,
                'D' => 5,
                _ => 0,
        } + if cs[0] != 'x' && cs[1] != 'x' { 2 } else { 0 };
        result
    }).sum())
}

fn part3(data: &str) -> Option<u32> {
    Some(data.chars().chunks(3).into_iter().map(|c| {
        let cs = c.collect::<Vec<_>>();
        let mut x = 0;
        let mut s: u32 = cs.iter().map(|m| {
            match m {
                'A' => 0,
                'B' => 1,
                'C' => 3,
                'D' => 5,
                _ => {
                    x += 1;
                    0
                },
            }
        }).sum();
        if x == 1 {
            s += 2
        } else if x == 0 {
            s += 6
        }
        s
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("ABBAC");
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part1_release() {
        let result = part1(include_str!("../data/everybody_codes_e2024_q01_p1.txt"));
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part2() {
        let result = part2("AxBCDDCAxD");
        assert_eq!(result, Some(28));
    }

    #[test]
    fn test_part2_release() {
        let result = part2(include_str!("../data/everybody_codes_e2024_q01_p2.txt"));
        println!("{}", result.unwrap());
    }


    #[test]
    fn test_part3() {
        let result = part3("xBxAAABCDxCC");
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_part3_release() {
        let result = part3(include_str!("../data/everybody_codes_e2024_q01_p3.txt"));
        println!("{}", result.unwrap());
    }
}
