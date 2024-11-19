#![allow(unused)]

use std::{collections::HashMap, hash::Hash};

fn part1(data: &str) -> Option<u64> {
    let generation_mapping = data.lines().map(|l| {
        let mut split = l.split(":");
        let key = split.next().unwrap();
        let value = split.next().unwrap().split(",").collect::<Vec<&str>>();
        (key, value)
    }).collect::<HashMap<&str, Vec<&str>>>();
    Some(get_population_after_days("A", 4, &generation_mapping))
}


fn part2(data: &str) -> Option<u64> {
    let generation_mapping = data.lines().map(|l| {
        let mut split = l.split(":");
        let key = split.next().unwrap();
        let value = split.next().unwrap().split(",").collect::<Vec<&str>>();
        (key, value)
    }).collect::<HashMap<&str, Vec<&str>>>();
    Some(get_population_after_days("Z", 10, &generation_mapping))
}

fn part3(data: &str) -> Option<u64> {
    let generation_mapping = data.lines().map(|l| {
        let mut split = l.split(":");
        let key = split.next().unwrap();
        let value = split.next().unwrap().split(",").collect::<Vec<&str>>();
        (key, value)
    }).collect::<HashMap<&str, Vec<&str>>>();

    let mut min = u64::MAX;
    let mut max = 0;

    generation_mapping.keys().clone().for_each(|k| {
        let population =  get_population_after_days(k, 20, &generation_mapping);
        if population < min {
            min = population;
        }
        if population > max {
            max = population;
        }
    });
    Some(max - min)
}

fn get_population_after_days(initial_type: &str, days: u32, generation_mapping: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut current_population = HashMap::new();
    current_population.insert(initial_type, 1);
    let mut new_population = HashMap::new();
    let mut sum = 0;
    for _ in 1..=days {
        sum = 0;
        for (key, value) in current_population.iter() {
            for v in generation_mapping.get(key).unwrap().iter() {
                let entry = new_population.entry(*v).or_insert(0);
                *entry += value;
                sum += value;
            }
        }
        current_population = new_population;
        new_population = HashMap::new();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("A:B,C
B:C,A
C:A");
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part1_release() {
        let result = part1(include_str!("../data/everybody_codes_e2024_q11_p1.txt"));
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part2() {
        let result = part2("");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part2_release() {
        let result = part2(include_str!("../data/everybody_codes_e2024_q11_p2.txt"));
        println!("{}", result.unwrap());
    }


    #[test]
    fn test_part3() {
        let result = part3("A:B,C
B:C,A,A
C:A");
        assert_eq!(result, Some(268815));
    }

    #[test]
    fn test_part3_release() {
        let result = part3(include_str!("../data/everybody_codes_e2024_q11_p3.txt"));
        println!("{}", result.unwrap());
    }
}
