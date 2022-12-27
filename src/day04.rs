use std::ops::RangeBounds;

struct Section {
    start: usize,
    end: usize
}

impl Section {
    fn overlaps(&self, other: &Section) -> bool {
        return (self.start..=self.end).contains(&other.start)
        && (self.start..=self.end).contains(&other.end);
    }

    fn partialy_overlaps(&self, other: &Section) -> bool {
        return (self.start..=self.end).contains(&other.start)
        || (self.start..=self.end).contains(&other.end);
    }
}


impl From<&str> for Section {
    fn from(s: &str) -> Self {
        Self {
            start: s.split("-").nth(0).expect("Cannot get 1st splitted range").parse::<usize>().unwrap(),
            end: s.split("-").nth(1).expect("Cannot get 1st splitted range").parse::<usize>().unwrap(),
        }
    }
}

fn parse(mut input: String) -> Vec<(Section, Section)> {
    input = input.trim_start().trim_end().to_string();
    let lines = input.lines().into_iter();
    let mut pair_elves_section_assigments_list = Vec::new();

    for (_id, line) in lines.enumerate() {
        let sections = line.trim().split(",").collect::<Vec<&str>>();
        eprintln!("sections: {:?}", sections);
        let (s1,s2) = (sections[0],sections[1]);
        pair_elves_section_assigments_list.push((Section::from(s1), Section::from(s2)));
    }

    pair_elves_section_assigments_list
}

pub fn part_one<S: Into<String>>(input: S) -> usize {
    let data = parse(input.into());
    let mut number_of_absolute_overlaping = 0;
    for (s1,s2) in data {
        if s1.overlaps(&s2) || s2.overlaps(&s1) {
            number_of_absolute_overlaping += 1;
        }
    }
    number_of_absolute_overlaping
}

pub fn part_two<S: Into<String>>(input: S) -> usize {
    let data = parse(input.into());
    let mut number_of_total_overlaping = 0;
    for (s1,s2) in data {
        let partialy_overlaps = s1.partialy_overlaps(&s2);
        let totally_overlaps = s1.overlaps(&s2) || s2.overlaps(&s1);
        if partialy_overlaps || totally_overlaps {
            number_of_total_overlaping += 1;
        }
    }
    number_of_total_overlaping
}

#[cfg(test)]
mod tests {
    use crate::day04;
    const URL: &str = "https://adventofcode.com/2022/day/4/input";

    #[test]
    fn test_01_part_one() {
        let input = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

        let result = day04::part_one(input);

        assert_eq!(result, 2)
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

        let result = day04::part_one(input);
        eprintln!("{}", result);

        assert_eq!(result, 530);
        Ok(())
    }

    #[test]
    fn test_03_part_two() {
        let input = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

        let result = day04::part_two(input);
        assert_eq!(result, 4)
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

        let result = day04::part_two(input);
        eprintln!("{}", result);

        assert_eq!(result, 0);
        Ok(())
    }
}