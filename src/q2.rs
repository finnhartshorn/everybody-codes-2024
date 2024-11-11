use std::collections::HashSet;
use itertools::Itertools;

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
    Some(0)
}


fn part3(data: &str) -> Option<u32> {
    Some(0)
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
        let result = part2("AxBCDDCAxD");
        assert_eq!(result, Some(28));
    }

    #[test]
    fn test_part2_release() {
        let result = part2(include_str!("../data/everybody_codes_e2024_q02_p2.txt"));
        println!("{}", result.unwrap());
    }


    #[test]
    fn test_part3() {
        let result = part3("xBxAAABCDxCC");
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_part3_release() {
        let result = part3(include_str!("../data/everybody_codes_e2024_q02_p3.txt"));
        println!("{}", result.unwrap());
    }
}
