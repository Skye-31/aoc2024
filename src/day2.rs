use aoc_runner_derive::aoc;

fn is_line_safe<'a, I>(mut numbers: I) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let mut prev = match numbers.next() {
        Some(num) => num.parse::<i16>().unwrap(),
        None => return false,
    };

    let mut ascending = true;
    let mut descending = true;

    for next in numbers {
        let next = next.parse::<i16>().unwrap();
        let diff = prev - next;

        if diff == 0 || diff.abs() > 3 {
            return false;
        }

        if prev > next {
            ascending = false;
        } else if prev < next {
            descending = false;
        }

        if !ascending && !descending {
            return false;
        }

        prev = next;
    }

    true
}

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let elements = line
                .split_whitespace();

            is_line_safe(elements)
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let elements: Vec<&str> = line
                .split_whitespace()
                .collect();

            is_line_safe(elements.iter().cloned())
                || (0..elements.len()).any(|index| {
                    let elements = elements.iter().enumerate()
                        .filter(|&(i, _)| index != i)
                        .map(|(_, &val)| val);

                    is_line_safe(elements)
                })
        })
        .count()
}
