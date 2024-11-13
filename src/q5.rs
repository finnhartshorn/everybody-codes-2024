#![allow(unused)]

use core::fmt;
use std::collections::HashSet;
use std::{collections::HashMap, hash::Hash, vec};
use std::collections::hash_map::Entry::{Vacant, Occupied};

fn part1(data: &str) -> Option<String> {
    let mut columns = [vec![], vec![], vec![], vec![]];
    data.lines().for_each(|l| {
        let mut i = 0;
        l.split_whitespace().for_each(|n| {
            columns[i].push(n.parse::<usize>().unwrap());
            i += 1;
        });
    });
    for round in 0..10 {
        let column_index = round % 4;
        let next_column_index = if column_index >= columns.len()-1 {0} else {column_index+1};
        let clapper = columns[column_index].remove(0);
        let new_column_len = columns[next_column_index].len();
        if clapper <= new_column_len {
            columns[next_column_index].insert(clapper-1, clapper)
        } else {
            columns[next_column_index].insert(new_column_len - (clapper - new_column_len), clapper)
        }
        println!("Round: {}", round+1);
        for i in 0..5 {
            let a = if columns[0].len() > i {&columns[0][i].to_string()} else {" "};
            let b = if columns[1].len() > i {&columns[1][i].to_string()} else {" "};
            let c = if columns[2].len() > i {&columns[2][i].to_string()} else {" "};
            let d = if columns[3].len() > i {&columns[3][i].to_string()} else {" "};
            println!("{a} {b} {c} {d}");
        }
    }
    Some(format!("{}{}{}{}", columns[0][0], columns[1][0], columns[2][0], columns[3][0]))
}


struct NumberAccum {
    count: u32,
    first_seen: usize,
}

fn part2(data: &str) -> Option<u64> {
    let mut columns = [vec![], vec![], vec![], vec![]];
    data.lines().for_each(|l| {
        let mut i = 0;
        l.split_whitespace().for_each(|n| {
            columns[i].push(n.parse::<usize>().unwrap());
            i += 1;
        });
    });
    let mut number_map: HashMap<u32, NumberAccum> = HashMap::new();
    let mut round = 0;
    loop {
        let column_index = round % 4;
        let next_column_index = if column_index >= columns.len()-1 {0} else {column_index+1};
        let clapper = columns[column_index].remove(0);
        let new_column_len = columns[next_column_index].len();
        let clamped_clapper = (clapper - 1) % (new_column_len * 2);
        if clamped_clapper < new_column_len {
            columns[next_column_index].insert(clamped_clapper, clapper)
        } else if clamped_clapper == new_column_len {
            columns[next_column_index].push(clapper)
        } else {
            columns[next_column_index].insert(new_column_len - (clamped_clapper - new_column_len), clapper)
        }

        let number = format!("{}{}{}{}", columns[0][0], columns[1][0], columns[2][0], columns[3][0]).parse::<u32>().unwrap();

        let mut number_accum  = match number_map.entry(number) {
            Vacant(entry) => entry.insert(NumberAccum{count: 0, first_seen: round+1}),
            Occupied(entry) => entry.into_mut(),
        };

        number_accum.count += 1;

        if number_accum.count == 2024 {
            return Some((round+1) as u64 * number as u64)
        }
        round += 1;
    }
    Some(0)
}


fn part3(data: &str) -> Option<u64> {
    let mut columns = [vec![], vec![], vec![], vec![]];
    data.lines().for_each(|l| {
        let mut i = 0;
        l.split_whitespace().for_each(|n| {
            columns[i].push(n.parse::<usize>().unwrap());
            i += 1;
        });
    });
    let mut seen_set = HashSet::new();
    let mut round = 0;
    let mut max = 0;
    loop {
        let column_index = round % 4;
        let next_column_index = if column_index >= columns.len()-1 {0} else {column_index+1};
        let clapper = columns[column_index].remove(0);
        let new_column_len = columns[next_column_index].len();
        let clamped_clapper = (clapper - 1) % (new_column_len * 2);
        if clamped_clapper < new_column_len {
            columns[next_column_index].insert(clamped_clapper, clapper)
        } else if clamped_clapper == new_column_len {
            columns[next_column_index].push(clapper)
        } else {
            columns[next_column_index].insert(new_column_len - (clamped_clapper - new_column_len), clapper)
        }

        let number = format!("{}{}{}{}", columns[0][0], columns[1][0], columns[2][0], columns[3][0]).parse::<u64>().unwrap();
        if number > max {
            max = number;
        }

        if !seen_set.insert(columns.clone()) {
            break;
        }

        round += 1;
    }
    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4");
        assert_eq!(result, Some("2323".to_string()));
    }

    #[test]
    fn test_part1_release() {
        let result = part1(include_str!("../data/everybody_codes_e2024_q05_p1.txt"));
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part2() {
        let result = part2("2 3 4 5
6 7 8 9");
        assert_eq!(result, Some(50877075));
    }

    #[test]
    fn test_part2_release() {
        let result = part2(include_str!("../data/everybody_codes_e2024_q05_p2.txt"));
        println!("{}", result.unwrap());
    }


    #[test]
    fn test_part3() {
        let result = part3("2 3 4 5
6 7 8 9");
        assert_eq!(result, Some(6584));
    }

    #[test]
    fn test_part3_release() {
        let result = part3(include_str!("../data/everybody_codes_e2024_q05_p3.txt"));
        println!("{}", result.unwrap());
    }
}
