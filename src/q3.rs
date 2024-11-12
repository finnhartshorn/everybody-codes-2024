#![allow(unused)]

use std::cmp::min;

fn part1(data: &str) -> Option<u32> {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    data.lines().for_each(|line| {
        grid.push(line.chars().map(|c|
        match c {
            '.' => -1,
            '#' => -2,
            _ => panic!("Invalid character"),
        }).collect());
    });
    let mut frontier = vec![(0,0)];
    let mut next_frontier = vec![];
    let mut level = 1;
    let mut s = 0;
    grid[0][0] = 0;
    loop {
        if frontier.is_empty() {
            if next_frontier.is_empty() {
                break;
            }
            frontier = next_frontier;
            next_frontier = vec![];
            level += 1;
        }
        let current_coord = frontier.pop().unwrap();
        get_cardinal_directions(current_coord, grid[0].len(), grid.len()).iter().for_each(|coord| {
            let value = grid[coord.1][coord.0];
            if value == -1 {
                grid[coord.1][coord.0] = 0;
                frontier.push(*coord);
            }
            if value == -2 {
                grid[coord.1][coord.0] = level;
                next_frontier.push(*coord);
                s += level;
            }
        })
    }

    Some(s as u32)
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


fn part2(data: &str) -> Option<u32> {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    data.lines().for_each(|line| {
        grid.push(line.chars().map(|c|
        match c {
            '.' => -1,
            '#' => -2,
            _ => panic!("Invalid character"),
        }).collect());
    });
    let mut frontier = vec![];
    let mut next_frontier = vec![];
    let mut level = 1;
    let mut s = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == -1 {
                grid[i][j] = 0;
                frontier.push((j, i));
            }
        }
    }
    loop {
        if frontier.is_empty() {
            if next_frontier.is_empty() {
                break;
            }
            frontier = next_frontier;
            next_frontier = vec![];
            level += 1;
        }
        let current_coord = frontier.pop().unwrap();
        get_cardinal_directions(current_coord, grid[0].len(), grid.len()).iter().for_each(|coord| {
            let value = grid[coord.1][coord.0];
            // if value == -1 {
                
            // }
            if value == -2 {
                grid[coord.1][coord.0] = level;
                next_frontier.push(*coord);
                s += level;
            }
        })
    }

    Some(s as u32)
}


fn part3(data: &str) -> Option<u32> {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    data.lines().for_each(|line| {
        grid.push(line.chars().map(|c|
        match c {
            '.' => -1,
            '#' => -2,
            _ => panic!("Invalid character"),
        }).collect());
    });
    let mut frontier = vec![];
    let mut next_frontier = vec![];
    let mut level = 1;
    let mut s = 0;
    let grid_x = grid[0].len();
    let grid_y = grid.len();
    // Handle border, simulating endless '.' outside of bounds
    for i in 0..grid_y {
        if grid[i][0] == -2 {
            grid[i][0] = 1;
            s += 1;
            next_frontier.push((0, i));
        }
        if grid[i][grid_x-1] == -2 {
            grid[i][grid_x-1] = 1;
            s += 1;
            next_frontier.push((grid_x-1, i));
        }
    }
    for i in 0..grid[0].len() {
        if grid[0][i] == -2 {
            grid[0][i] = 1;
            s += 1;
            next_frontier.push((i, 0));
        }
        if grid[grid.len()-1][i] == -2 {
            grid[grid_y-1][i] = 1;
            s += 1;
            next_frontier.push((i, grid_y-1));
        }
    }
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == -1 {
                grid[i][j] = 0;
                frontier.push((j, i));
            }
        }
    }
    loop {
        if frontier.is_empty() {
            if next_frontier.is_empty() {
                break;
            }
            frontier = next_frontier;
            next_frontier = vec![];
            level += 1;
        }
        let current_coord = frontier.pop().unwrap();
        get_all_directions(current_coord, grid[0].len(), grid.len()).iter().for_each(|coord| {
            let value = grid[coord.1][coord.0];
            if value == -2 {
                grid[coord.1][coord.0] = level;
                next_frontier.push(*coord);
                s += level;
            }
        })
    }

    Some(s as u32)
}

fn get_all_directions(coord: (usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut results = vec![];
    if coord.0 < max_x-1 {
        results.push((coord.0+1, coord.1));
    }
    if coord.1 < max_y-1 {
        results.push((coord.0, coord.1+1));
    }
    if coord.0 < max_x-1 && coord.1 < max_y-1 {
        results.push((coord.0+1, coord.1+1));
    }
    if coord.0 != 0 {
        results.push((coord.0-1, coord.1));
    }
    if coord.1 != 0 {
        results.push((coord.0, coord.1-1));
    }
    if coord.0 != 0 && coord.1 != 0 {
        results.push((coord.0-1, coord.1-1));
    }
    if coord.0 != 0 && coord.1 < max_y-1 {
        results.push((coord.0-1, coord.1+1));
    }
    if coord.0 < max_x-1 && coord.1 != 0 {
        results.push((coord.0+1, coord.1-1));
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("..........
..###.##..
...####...
..######..
..######..
...####...
..........");
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part1_release() {
        let result = part1(include_str!("../data/everybody_codes_e2024_q03_p1.txt"));
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part2() {
        let result = part2("..........
..###.##..
...####...
..######..
..######..
...####...
..........");
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part2_release() {
        let result = part2(include_str!("../data/everybody_codes_e2024_q03_p2.txt"));
        println!("{}", result.unwrap());
    }


    #[test]
    fn test_part3() {
        let result = part3("..........
..###.##..
...####...
..######..
..######..
...####...
..........");
        assert_eq!(result, Some(29));
    }

    #[test]
    fn test_part3_release() {
        let result = part3(include_str!("../data/everybody_codes_e2024_q03_p3.txt"));
        println!("{}", result.unwrap());
    }
}
