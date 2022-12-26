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