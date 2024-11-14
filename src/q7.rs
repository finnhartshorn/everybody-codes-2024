#![allow(unused)]

use std::collections::HashMap;

use itertools::Itertools;

fn part1(data: &str) -> Option<String> {
    Some(data.lines().map(|l| {
        let mut iter = l.split(":");
        let key = iter.next().unwrap().chars().next().unwrap();
        let values: Vec<char> = iter.next().unwrap().split(",").map(|v| v.chars().next().unwrap()).collect();
        let mut s = 10;
        let essence: i32 = (0..10).map(|i| {
            let clamped_index = i % values.len();
            match values[clamped_index] {
                '+' => {
                    s += 1;
                    s
                },
                '-' => {
                    if s != 0 {
                        s -= 1;
                    }
                    s
                },
                '=' => s,
                _ => panic!("Invalid value")
            }
        }).sum();
        println!("{} {}", key, essence);
        (key, essence)
    }).sorted_by_key(|f| f.1).map(|e| e.0).rev().collect())
}


fn part2(data: &str, data2: &str) -> Option<String> {
    let mut race_iter = data2.lines();
    let mut race = Vec::new();
    let mut temp_second = Vec::new();
    race.extend(race_iter.next().unwrap().chars());
    race_iter.for_each(|l| {
        let line = l.split_whitespace().collect::<Vec<&str>>();
        if line.len() == 2 {
            temp_second.push(line[0].chars().next().unwrap());
            race.push(line[1].chars().next().unwrap());
        } else {
            race.extend(l.chars().rev());
        }
    });
    race.extend(temp_second.iter().rev());

    println!("{:?}", race);

    Some(data.lines().map(|l| {
        let mut iter = l.split(":");
        let key = iter.next().unwrap().chars().next().unwrap();
        let values: Vec<char> = iter.next().unwrap().split(",").map(|v| v.chars().next().unwrap()).collect();
        let mut power = 10;
        let mut essence: i32 = 0;
        let mut laps = 0;
        let mut steps = 0;
        while laps < 10 {
            let race_char = race[(steps+1) % race.len()];
            let plan_char = values[steps % values.len()];
            if match race_char {
                'S' => {
                    laps += 1;
                    true
                },
                '-' => {
                    if power != 0 {
                        power -= 1;
                    }
                    false
                },
                '+' => {
                    power += 1;
                    false
                },
                '=' => {
                    true
                }
                _ => panic!("Invalid value")
            } {
                match plan_char {
                    '+' => {
                        power += 1;
                    },
                    '-' => {
                        if power != 0 {
                            power -= 1;
                        }
                    },
                    '=' => (),
                    _ => panic!("Invalid value")
                }
            }

            essence += power;
            steps += 1;
            // println!("{} {}", steps, power);
        }
        (key, essence)
    }).sorted_by_key(|f| f.1).map(|e| e.0).rev().collect())
}


fn part3(data: &str, data2: &str) -> Option<u32> {
    let race = calc_map(data2);

    println!("{:?}", race);

    let opponent_strategy: Vec<char> = data.split(":").nth(1).unwrap().split(",").map(|c| c.chars().next().unwrap()).collect(); 
    let opponent_essence = calc_essence(2024, &race, &opponent_strategy);
    println!("{:?}", opponent_essence);
    let mut index = 0;

    Some(['-', '-', '-', '+', '+', '+', '+', '+', '=', '=', '='].iter().permutations(11).unique().map(|s| {
        let essence = calc_essence(2024, &race, &s.iter().map(|c| **c).collect::<Vec<char>>());
        index += 1;
        if essence > opponent_essence {
            1
        } else {
            0
        }
    }).sum())
}

fn calc_essence(max_laps: usize, race: &[char], values: &[char]) -> u64 {
    let mut power = 10;
    let mut essence: u64 = 0;
    let mut laps = 0;
    let mut steps = 0;
    while laps < max_laps {
        let race_char = race[(steps) % race.len()];
        let plan_char = values[steps % values.len()];
        if match race_char {
            'S' => {
                laps += 1;
                true
            },
            '-' => {
                if power != 0 {
                    power -= 1;
                }
                false
            },
            '+' => {
                power += 1;
                false
            },
            '=' => {
                true
            }
            _ => panic!("Invalid value")
        } {
            match plan_char {
                '+' => {
                    power += 1;
                },
                '-' => {
                    if power != 0 {
                        power -= 1;
                    }
                },
                '=' => (),
                _ => panic!("Invalid value")
            }
        }

        essence += power;
        steps += 1;
    }
    essence
}

fn calc_map(data: &str) -> Vec<char> {
    let map_grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let mut prev_coord = (0, 0);
    let mut current_coord = (1, 0);
    let mut result = vec![map_grid[current_coord.1][current_coord.0]];
    while map_grid[current_coord.1][current_coord.0] != 'S' {
        for coord in get_cardinal_directions(current_coord, map_grid[0].len(), map_grid.len()) { //}.iter().for_each(|coord| {
            if coord == prev_coord {
                continue;
            }
            let value = map_grid[coord.1][coord.0];
            if value == ' ' {
                continue;
            }
            prev_coord = current_coord;
            current_coord = coord;
            result.push(value);
            break;
        }
    }
    result
}

fn get_cardinal_directions(coord: (usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut results = vec![];
    if coord.0 < max_x-1 {
        results.push((coord.0+1, coord.1));
    }
    if coord.1 < max_y-1 {
        results.push((coord.0, coord.1+1));
    }
    if coord.0 != 0 {
        results.push((coord.0-1, coord.1));
    }
    if coord.1 != 0 {
        results.push((coord.0, coord.1-1));
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+");
        assert_eq!(result, Some("BDCA".to_string()));
    }

    #[test]
    fn test_part1_release() {
        let result = part1(include_str!("../data/everybody_codes_e2024_q07_p1.txt"));
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part2() {
        let result = part2("A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+", "S+===
-   +
=+=-+");
        assert_eq!(result, Some("DCBA".to_string()));
    }

    #[test]
    fn test_part2_release() {
        let result = part2(include_str!("../data/everybody_codes_e2024_q07_p2.txt"), "S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--
-                                                                     -
=                                                                     =
+                                                                     +
=                                                                     +
+                                                                     =
=                                                                     =
-                                                                     -
--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-");
        println!("{}", result.unwrap());
    }


    #[test]
    fn test_part3() {
        let result = part3("", "");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part3_release() {
        let result = part3(include_str!("../data/everybody_codes_e2024_q07_p3.txt"), "S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=       
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =          
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-");
        println!("{}", result.unwrap());
    }
}
