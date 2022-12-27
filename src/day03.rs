use counter::Counter;

fn parse_part_one(mut input: String) -> Vec<(String, String)> {
    input = input.trim_start().trim_end().to_string();
    let lines = input.lines().into_iter();
    let mut rucksack_item_list: Vec<(String, String)> = Vec::new();
    for line in lines {
        let trimmed_line = line.trim();
        let len_of_trimmed_line = trimmed_line.len();
        let (pocket1, pocket2) =
            trimmed_line.split_at(len_of_trimmed_line / 2 + len_of_trimmed_line % 2);
        rucksack_item_list.push((pocket1.into(), pocket2.into()));
    }
    rucksack_item_list
}

pub fn part_one<S: Into<String>>(input: S) -> usize {
    let data = parse_part_one(input.into());
    let mut sum_of_priorities_for_letter_in_common = 0;
    let mut v = vec![];
    for (a, b) in data {
        let a_count = a.chars().collect::<Counter<_>>();
        let b_count = b.chars().collect::<Counter<_>>();
        let mut priority = 0;
        eprintln!(
            "(a: {:?}, b: {:?}) {:?}",
            a,
            b,
            a_count.clone() & b_count.clone()
        );
        for c in (a_count & b_count).keys() {
            priority = get_priority_from_char(c);
            v.push(priority);
        }
        sum_of_priorities_for_letter_in_common += priority;
    }
    eprintln!("{:?}", v);
    sum_of_priorities_for_letter_in_common
}

fn get_priority_from_char(c: &char) -> usize {
    (if c.is_uppercase() {
        (*c as usize) - ('A' as usize) + 26
    } else if c.is_lowercase() {
        (*c as usize) - ('a' as usize)
    } else {
        panic!("Error: char {:?} is not alphabetic", c);
    }) + 1
}

fn parse_part_two(mut input: String) -> Vec<(String, String, String)> {
    input = input.trim_start().trim_end().to_string();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut rucksack_elves_group_list: Vec<(String, String, String)> = Vec::new();

    for i in (0..lines.len()).step_by(3) {
        let elves_sacks = (
            lines[i].trim().into(),
            lines[i + 1].trim().into(),
            lines[i + 2].trim().into(),
        );
        rucksack_elves_group_list.push(elves_sacks);
    }
    rucksack_elves_group_list
}

pub fn part_two<S: Into<String>>(input: S) -> usize {
    let data = parse_part_two(input.into());
    eprintln!("Data: {:?}", data);

    let mut sum_of_badge_priority = 0;

    for elves_sack in data {
        let sack0 = elves_sack.0.chars().collect::<Counter<_>>();
        let sack1 = elves_sack.1.chars().collect::<Counter<_>>();
        let sack2 = elves_sack.2.chars().collect::<Counter<_>>();

        let badge_in_common = sack0 & sack1 & sack2;

        for c in badge_in_common.keys() {
            sum_of_badge_priority += get_priority_from_char(c);
        }
        eprintln!("Bagde: {:?}", badge_in_common);
    }
    sum_of_badge_priority
}


#[cfg(test)]
mod tests {
    use crate::day03;
    const URL: &str = "https://adventofcode.com/2022/day/3/input";

    #[test]
    fn test_01_part_one() {
        let input = "AvJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        let result = day03::part_one(input);

        assert_eq!(result, 157)
    }

    #[test]
    // #[ignore = "Not implemented"]
    fn test_02_part_one() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client.get(URL)
        .header("cookie", crate::util::get_auth_cookie_session_from_envfile())
        .send().expect("Unable to get from url");

        let input = response.text().expect("Unable to retrieve text from response");
        eprintln!("{}", input);

        let result = day03::part_one(input);
        eprintln!("{}", result);

        assert_eq!(result, 8153);
        Ok(())
    }

    #[test]
    fn test_03_part_one() {
        let input = "AvJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        let result = day03::part_two(input);

        assert_eq!(result, 70)
    }

    #[test]
    // #[ignore = "Not implemented"]
    fn test_04_part_two() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client.get(URL)
        .header("cookie", crate::util::get_auth_cookie_session_from_envfile())
        .send().expect("Unable to get from url");

        let input = response.text().expect("Unable to retrieve text from response");
        eprintln!("{}", input);

        let result = day03::part_two(input);
        eprintln!("{}", result);

        assert_eq!(result, 2342);
        Ok(())
    }
}