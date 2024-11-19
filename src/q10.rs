#![allow(unused)]

use std::{collections::HashSet, hash::Hash, string};

use itertools::Itertools;

fn part1(data: &str) -> Option<String> {
    let mut column_sets = [HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()];
    let mut row_sets = [HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()];
    let mut line_i = 0;
    let mut j = 0;
    data.lines().for_each(|l| {
        if line_i < 2 || line_i > 5 {
            let mut i = 0;
            let mut column_i = 0;
            l.chars().for_each(|c| {
                if i > 1 && i < 6 {
                    column_sets[column_i].insert(c);
                    column_i += 1;
                }
                i += 1;
            });
        } else {
            let mut i = 0;
            l.chars().for_each(|c| {
                // println!("{}", c);
                if i < 2 || i > 5 {
                    row_sets[j].insert(c);
                }
                i += 1;
            });
            j += 1;
        }
        line_i += 1;
    });
    println!("{:?}", column_sets);
    println!("{:?}", row_sets);
    let mut string_buffer = String::new();
    let mut i = 1;
    let mut sum = 0;
    for row in row_sets.iter() {
        for column in column_sets.iter() {
            for c in column.intersection(row) {
                let value = c.to_ascii_lowercase() as u8 - 96;
                sum += value * i;
                i += 1;
            }
        }
    }
    Some(string_buffer)
}


fn part2(data: &str) -> Option<u32> {
    let mut set_data = vec![([HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()], [HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()]); 105];
    // let mut global_line_i = 0;
    // let mut j = 0;
    let mut y = 0;
    data.lines().for_each(|l| {
        let mut x = 0;
        l.chars().for_each(|c| {
            add_to_set(&mut set_data, x, y, c);
            x += 1;
        });
        y += 1;
        // println!("{:?}", set_data);
        // panic!();
        // if y > 3 {
        //     panic!();
        // }
    });


    //     let line_i = global_line_i % 9;
    //     let set = global_line_i/9;
    //     // let row_i = 
    //     println!("{global_line_i} {line_i} {set}");
    //     println!("{:?}", set_data[set]);
    //     if line_i < 2 || line_i > 5 {
    //         let mut i = 0;
    //         let mut column_i = 0;
    //         l.chars().for_each(|c| {
    //             if i > 1 && i < 6 {
    //                 set_data[set].0[column_i].insert(c);
    //                 // column_sets[column_i].insert(c);
    //                 column_i += 1;
    //             }
    //             i += 1;
    //         });
    //     } else {
    //         let mut i = 0;
    //         l.chars().for_each(|c| {
    //             // println!("{}", c);
    //             if i < 2 || i > 5 {
    //                 set_data[set].1[j].insert(c);
    //                 // row_sets[j].insert(c);
    //             }
    //             i += 1;
    //         });
    //         j += 1;
    //     };
    //     global_line_i += 1;
    // });
    // println!("{:?}", set_data);
    let mut sum = 0;
    for set in set_data.iter() {
        let mut i = 1;
        let column_sets = &set.0;
        let row_sets = &set.1;
        for row in set.1.iter() {
            for column in set.0.iter() {
                let c = column.intersection(row).exactly_one().unwrap();
                let value = c.to_ascii_lowercase() as u8 - 96;
                println!("{c} {i} {value} {}", value as u32 * i);
                sum += value as u32 * i;
                i += 1;
            }
        }
    }
    Some(sum)
}

fn add_to_set(sets: &mut Vec<([HashSet<char>; 4], [HashSet<char>; 4])>, x: usize, y: usize, value: char) {
    let local_x = x % 9;
    let local_y = y % 9;
    let set_index = (x / 9) + ((y / 9) * 15);

    let in_centre_y = (2..6).contains(&local_y);
    let in_centre_x = (2..6).contains(&local_x);

    if !in_centre_y && in_centre_x && local_y != 8 {
        println!("Adding to column set ({x}, {y}) ({local_x}, {local_y}) {set_index} {value}");
        sets[set_index].0[local_x - 2].insert(value);
    } else if in_centre_y && !in_centre_x && local_x != 8 {
        println!("Adding to row set ({x}, {y}) ({local_x}, {local_y}) {set_index} {value}");
        sets[set_index].1[local_y - 2].insert(value);
    }
}

fn part3(data: &str) -> Option<u32> {
    Some(0)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("**PCBS**
**RLNW**
BV....PT
CR....HZ
FL....JW
SG....MN
**FTZV**
**GMJH**");
        assert_eq!(result, Some("PTBVRCZHFLJWGMNS".to_string()));
    }

    #[test]
    fn test_part1_release() {
        let result = part1(include_str!("../data/everybody_codes_e2024_q10_p1.txt"));
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part2() {
        let result = part2("");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part2_release() {
        let result = part2(include_str!("../data/everybody_codes_e2024_q10_p2.txt"));
        println!("{}", result.unwrap());
    }


    #[test]
    fn test_part3() {
        let result = part3("");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part3_release() {
        let result = part3(include_str!("../data/everybody_codes_e2024_q02_p3.txt"));
        println!("{}", result.unwrap());
    }
}
