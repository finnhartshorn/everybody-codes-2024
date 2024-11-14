#![allow(unused)]

use std::cmp::min;

use itertools::max;

fn part1(data: &str) -> Option<u32> {
    let num_blocks = data.parse::<u32>().unwrap();
    let mut i = 1;
    loop {
        let total_blocks = i * i;
        if total_blocks >= num_blocks {
            let width = (i * 2) - 1;
            let required_blocks = total_blocks - num_blocks;
            return Some(width * required_blocks);
        }
        i += 1;
    }
    Some(0)
}


fn part2(data: &str, accolytes: u64, blocks: u64) -> Option<u64> {
    let num_priests = data.parse::<u64>().unwrap();
    let mut width = 1;
    let mut prev_layer = 1;
    let mut total_blocks: u64 = 1;
    while total_blocks < blocks {
        width += 2;
        let next_layer = (prev_layer * num_priests) % accolytes;
        total_blocks += next_layer * width;
        prev_layer = next_layer;
    }
    Some(width * (total_blocks - blocks))
}

fn part3(data: &str, accolytes: u64, blocks: u64) -> Option<u64> {
    let num_priests = data.parse::<u64>().unwrap();
    let mut width = 1;
    let mut prev_layer = 1;
    let mut total_blocks: u64 = 1;
    let mut columns = vec![1];
    loop {
        width += 2;
        let next_layer = (prev_layer * num_priests) % accolytes + accolytes;
        total_blocks += next_layer * width;
        prev_layer = next_layer;
        columns.insert(0, 0);
        columns.push(0);
        for i in 0..columns.len() {
            columns[i] += next_layer;
        }
        if total_blocks < blocks {
            continue;
        }
        let mut actual_total_blocks = total_blocks;
        for i in 1..columns.len()-1 {
            let max_remove = min(columns[i-1]-1, columns[i+1]-1);
            let try_remove = min((width * num_priests * columns[i]) % accolytes, max_remove);
            actual_total_blocks -= try_remove;
        }
        if actual_total_blocks >= blocks {
            return Some(actual_total_blocks - blocks)
        }
    }
    Some(0)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("13");
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part1_release() {
        let result = part1(include_str!("../data/everybody_codes_e2024_q08_p1.txt"));
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part2() {
        let result = part2("3", 5, 50);
        assert_eq!(result, Some(27));
    }

    #[test]
    fn test_part2_release() {
        let result = part2(include_str!("../data/everybody_codes_e2024_q08_p2.txt"), 1111, 20240000);
        println!("{}", result.unwrap());
    }


    #[test]
    fn test_part3() {
        let result = part3("2", 5, 125820923);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part3_release() {
        let result = part3(include_str!("../data/everybody_codes_e2024_q08_p3.txt"), 10, 202400000000);
        println!("{}", result.unwrap());
    }
}
