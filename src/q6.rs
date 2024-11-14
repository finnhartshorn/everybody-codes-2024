#![allow(unused)]

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn part1(data: &str) -> Option<String> {
    let mut unique_paths: HashMap::<usize, &String> = HashMap::new();
    let mut seen_lengths: HashSet<usize> = HashSet::new();
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    data.lines().for_each(|l| {
        let mut iter = l.split(":");
        let key = iter.next().unwrap();
        let values = iter.next().unwrap().split(",").collect();
        tree.insert(key, values);
    });
    let key_size = tree.get("RR").unwrap().len();
    let paths = path_inner(&mut tree, "RR", "RR");
    paths.iter().map(|p| (p.len(), p)).for_each(|l| {
        if !seen_lengths.contains(&l.0) {
            unique_paths.insert(l.0, l.1);
            seen_lengths.insert(l.0);
        } else {
            unique_paths.remove(&l.0);
        }
    });
    Some(unique_paths.values().next().unwrap().split(",").collect())
}

fn path_inner(tree: &mut HashMap<&str, Vec<&str>>, key: &str, current_path: &str) -> Vec<String> {
    let Some(next_node) = tree.get(key) else {
        return vec![];
    };
    let mut result: Vec<String> = vec![];
    next_node.clone().iter().for_each(|n| {
        let next_path = format!("{},{}", current_path, n);
        if *n == "@" {
            result.push(next_path);
            return;
        } else if *n == "BUG"{
            return
        }
        let child_paths = path_inner(tree, n, &next_path);
        result.extend(child_paths);
    });
    result
}


fn part2(data: &str) -> Option<String> {
    let mut unique_paths: HashMap::<usize, &String> = HashMap::new();
    let mut seen_lengths: HashSet<usize> = HashSet::new();
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    data.lines().for_each(|l| {
        let mut iter = l.split(":");
        let key = iter.next().unwrap();
        let values = iter.next().unwrap().split(",").collect();
        tree.insert(key, values);
    });
    let key_size = tree.get("RR").unwrap().len();
    let paths = path_inner(&mut tree, "RR", "RR");
    paths.iter().map(|p| (p.len(), p)).for_each(|l| {
        if !seen_lengths.contains(&l.0) {
            unique_paths.insert(l.0, l.1);
            seen_lengths.insert(l.0);
        } else {
            unique_paths.remove(&l.0);
        }
    });
    Some(unique_paths.values().next().unwrap().split(",").map(|s| s.chars().next().unwrap()).collect())
}


fn part3(data: &str) -> Option<String> {
    let mut unique_paths: HashMap::<usize, &String> = HashMap::new();
    let mut seen_lengths: HashSet<usize> = HashSet::new();
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    data.lines().for_each(|l| {
        let mut iter = l.split(":");
        let key = iter.next().unwrap();
        let values = iter.next().unwrap().split(",").collect();
        tree.insert(key, values);
    });
    let key_size = tree.get("RR").unwrap().len();
    let paths = path_inner(&mut tree, "RR", "RR");
    paths.iter().map(|p| (p.len(), p)).for_each(|l| {
        if !seen_lengths.contains(&l.0) {
            unique_paths.insert(l.0, l.1);
            seen_lengths.insert(l.0);
        } else {
            unique_paths.remove(&l.0);
        }
    });
    Some(unique_paths.values().next().unwrap().split(",").map(|s| s.chars().next().unwrap()).collect())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@");
        assert_eq!(result, Some("RRB@".to_string()));
    }

    #[test]
    fn test_part1_release() {
        let result = part1(include_str!("../data/everybody_codes_e2024_q06_p1.txt"));
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part2() {
        let result = part2("RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@");
        assert_eq!(result, Some("RB@".to_string()));
    }

    #[test]
    fn test_part2_release() {
        let result = part2(include_str!("../data/everybody_codes_e2024_q06_p2.txt"));
        println!("{}", result.unwrap());
    }


    #[test]
    fn test_part3() {
        let result = part3("RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@");
        assert_eq!(result, Some("RB@".to_string()));
    }

    #[test]
    fn test_part3_release() {
        let result = part3(include_str!("../data/everybody_codes_e2024_q06_p3.txt"));
        println!("{}", result.unwrap());
    }
}
