use aoc_runner_derive::aoc;

fn is_line_safe<'a, I>(mut numbers: I) -> Option<usize>
where
    I: Iterator<Item = &'a str>,
{
    let mut prev = numbers.next().unwrap().parse::<i16>().unwrap();

    let mut ascending = true;
    let mut descending = true;

    let mut idx = 1;

    for next in numbers {
        let next = next.parse::<i16>().unwrap();
        let diff = prev - next;

        if diff == 0 || diff.abs() > 3 {
            return Some(idx);
        }

        if prev > next {
            ascending = false;
        } else if prev < next {
            descending = false;
        }

        if !ascending && !descending {
            return Some(idx);
        }

        prev = next;
        idx += 1;
    }

    None
}

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let elements = line.split_whitespace();

            is_line_safe(elements).is_none()
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let elements: Vec<&str> = line.split_whitespace().collect();

            // perfect row
            let Some(index) = is_line_safe(elements.iter().cloned()) else {
                return true;
            };

            // eliminate the current number
            let Some(index) = try_with(&elements, index) else {
                return true;
            };

            // special case for the 1st->2nd number decreasing, then all others increasing
            if index == 2 && try_with(&elements, 0).is_none() {
                return true;
            }

            // eliminate the previous number, in case that was problematic
            try_with(&elements, index - 1).is_none()
        })
        .count()
}

fn try_with(elements: &Vec<&str>, index: usize) -> Option<usize> {
    let numbers = elements
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != index)
        .map(|(_, &val)| val);

    is_line_safe(numbers)
}

#[cfg(test)]
mod tests {
    use crate::day2::*;

    const INPUT: &str = include_str!("../input/2024/day2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 213);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("83 81 82 83 85 87 90 92"), 1);
        assert_eq!(part2("54 51 54 55 57 58"), 1);

        assert_eq!(part2(INPUT), 285);
    }
}
