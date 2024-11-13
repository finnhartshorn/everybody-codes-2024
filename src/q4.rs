#![allow(unused)]

use itertools::Itertools;

fn part1(data: &str) -> Option<u32> {
    // Might come back to this, try and do in one pass
    let iterator = data.lines().map(|l| l.trim().parse::<u32>().unwrap());
    let min = iterator.clone().min().unwrap();
    Some(iterator.map(|i| i - min).sum())
}


fn part2(data: &str) -> Option<u32> {
    part1(data)
}


fn part3(data: &str) -> Option<u32> {
    let iterator = data.lines().map(|l| l.trim().parse::<u32>().unwrap());
    let sorted_list = iterator.clone().sorted().collect::<Vec<u32>>();
    let median = sorted_list[sorted_list.len()/2];
    println!("{} {} {} {}", sorted_list.len(), sorted_list[sorted_list.len()/2], sorted_list[sorted_list.len()/2 + 1], median);
    Some(iterator.map(|i| median.abs_diff(i)).sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("3
4
7
8");
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part1_release() {
        let result = part1(include_str!("../data/everybody_codes_e2024_q04_p1.txt"));
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part2() {
        let result = part2("3
4
7
8");
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part2_release() {
        let result = part2(include_str!("../data/everybody_codes_e2024_q04_p2.txt"));
        println!("{}", result.unwrap());
    }


    #[test]
    fn test_part3() {
        let result = part3("2
4
5
6
8");
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part3_release() {
        let result = part3(include_str!("../data/everybody_codes_e2024_q04_p3.txt"));
        println!("{}", result.unwrap());
    }
}
