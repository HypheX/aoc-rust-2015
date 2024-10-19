advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut accumulator = 0_i32;
    for ch in input.chars() {
        match ch {
            '(' => accumulator += 1,
            ')' => accumulator -= 1,
            _ => {}
        }
    }
    Some(accumulator)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut accumulator = 0_i32;
    for (index, ch) in input.chars().enumerate() {
        match ch {
            '(' => accumulator += 1,
            ')' => accumulator -= 1,
            _ => {}
        }
        if accumulator < 0 {
            return Some(index + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
