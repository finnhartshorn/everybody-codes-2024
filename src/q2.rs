#![allow(unused)]

use std::cmp::min;
use std::collections::{HashMap, HashSet};

fn part1(data: &str) -> Option<u32> {
    let mut lines_iter = data.lines();
    let mut runes_2 = HashSet::new();
    let mut runes_3 = HashSet::new();
    lines_iter.next().unwrap().strip_prefix("WORDS:")?.split(',').map(|x| x.trim()).for_each(|r| {
       if r.len() == 2 {
           runes_2.insert(r.to_string());
       } else if r.len() == 3 {
           runes_3.insert(r.to_string());
       }
    });
    lines_iter.next();
    let words = lines_iter.next().unwrap().trim();
    let mut s = 0;
    let mut start = 0;
    let mut end = 2;
    loop {
        if runes_2.contains(&words[start..end]) {
            s += 1;
        }
        if end >= words.len() {
            break;
        }
        end += 1;
        if runes_3.contains(&words[start..end]) {
            s += 1;
        }
        start += 1;
    }
    Some(s)
}


fn part2(data: &str) -> Option<u32> {
    let mut max_run = 0;
    let mut set_map: HashMap<usize, HashSet<&str>> = HashMap::new();

    let mut lines_iter = data.lines();

    lines_iter.next().unwrap().strip_prefix("WORDS:")?.split(',').map(|x| x.trim()).for_each(|r| {
        set_map.entry(r.len()).or_default().insert(r); //TODO: Maybe inserting the reverse here is faster?
        if r.len() > max_run {
            max_run = r.len();
        }
    });

    lines_iter.next();

    let mut s = 0;
    lines_iter.for_each(|l| {
        let mut start = 0;
        let mut end = 1;
        let mut seen_vec = vec![false; l.len()]; // Bitset/Bitvec would probably be faster here
        while start < l.len() {
            loop {
                let current_len = end - start;
                if let Some(inner_set) = set_map.get(&current_len) {
                    let substring = &l[start..end];
                    if inner_set.contains(substring) || inner_set.contains(&substring.chars().rev().collect::<String>() as &str){
                        for i in start..end {
                            if !seen_vec[i] {
                                seen_vec[i] = true;
                                s += 1
                            }
                        }
                    }
                }
                if current_len > max_run || end >= l.len() {
                    start += 1;
                    end = start + 1;
                    break;
                }
                end += 1;
            }
        }
    });


    Some(s as u32)
}


fn part3(data: &str) -> Option<u32> {
    let mut max_run = 0;
    let mut set_map: HashMap<usize, HashSet<&str>> = HashMap::new();

    let mut lines_iter = data.lines();

    lines_iter.next().unwrap().strip_prefix("WORDS:")?.split(',').map(|x| x.trim()).for_each(|r| {
        set_map.entry(r.len()).or_insert(HashSet::new()).insert(r); //TODO: Maybe inserting the reverse here is faster?
        if r.len() > max_run {
            max_run = r.len();
        }
    });

    lines_iter.next();

    let mut s = 0;
    let mut x = 0;
    let mut y = 0;
    let mut seen: Vec<Vec<bool>> = Vec::new();
    let mut value: Vec<&str> = Vec::new();
    lines_iter.for_each(|l| {
        x = l.len();
        y += 1;
        seen.push(vec![false; x]);
        value.push(l);
    });

    for l_index in 0..y {
        let l = value[l_index];
        let mut start = 0;
        let mut end = 1;
        // let mut seen_vec = vec![false; l.len()]; // Bitset/Bitvec would probably be faster here
        while start < l.len() {
            loop {
                let current_len = end - start;
                if let Some(inner_set) = set_map.get(&current_len) {
                    let first_part_substr = &l[start..min(end, l.len())];
                    let substring = if end > l.len() {
                        &(first_part_substr.to_owned() + &l[0..end-l.len()])
                    } else {
                        first_part_substr
                    };
                    if inner_set.contains(substring) || inner_set.contains(&substring.chars().rev().collect::<String>() as &str){
                        for i in start..end {
                            let j = if i >= l.len() {i - l.len()} else {i};
                            if !seen[l_index][j] {
                                seen[l_index][j] = true;
                                s += 1
                            }
                        }
                    }
                }
                if current_len > max_run {
                    start += 1;
                    end = start + 1;
                    break;
                }
                end += 1;
            }
        }
    };

    for i in 0..x {
        for start in 0..y {
            for end in start+1..y+1 {
                let current_len = end - start;
                // println!("{}, {}, {} ({})", i, start, end, current_len);
                if let Some(inner_set) = set_map.get(&current_len) {
                    let substring = get_vertical_substr(&value, i, start, end-start);
                    // println!("{} {}", current_len, substring);
                    if inner_set.contains(&substring as &str) || inner_set.contains(&substring.chars().rev().collect::<String>() as &str) {
                        for j in start..end {
                            if seen[j][i] == false {
                                seen[j][i] = true;
                                s += 1
                            }
                        }
                    }
                }
                if current_len > max_run {
                    break;
                }
            }
        }
    }

    Some(s as u32)
}

fn get_vertical_substr(values: &Vec<&str>, x: usize, y: usize, len: usize) -> String {
    let mut str_buffer = Vec::new();
    for i in y..y+len {
        str_buffer.push(values[i].as_bytes()[x] as char) //Assumes all ascii
    };
    str_buffer.iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE");
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part1_release() {
        let result = part1(include_str!("../data/everybody_codes_e2024_q02_p1.txt"));
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part2() {
        let result = part2("WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END");
        assert_eq!(result, Some(37));
    }

    #[test]
    fn test_part2_release() {
        let result = part2(include_str!("../data/everybody_codes_e2024_q02_p2.txt"));
        println!("{}", result.unwrap());
    }


    #[test]
    fn test_part3() {
        let result = part3("WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL");
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part3_release() {
        let result = part3(include_str!("../data/everybody_codes_e2024_q02_p3.txt"));
        println!("{}", result.unwrap());
    }
}
