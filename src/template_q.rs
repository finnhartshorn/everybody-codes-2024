#![allow(unused)]

fn part1(data: &str) -> Option<u32> {
    Some(0)
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
        let result = part1("");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part1_release() {
        let result = part1(include_str!("../data/everybody_codes_e2024_q02_p1.txt"));
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part2() {
        let result = part2("");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part2_release() {
        let result = part2(include_str!("../data/everybody_codes_e2024_q02_p2.txt"));
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
