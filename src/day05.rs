use std::{collections::BTreeMap};

#[derive(Debug, Clone)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        use regex::Regex;
        let re = Regex::new(r"move (?P<quantity>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
        let caps = re.captures(s).unwrap();

        Self {
            quantity: caps["quantity"]
                .parse::<usize>()
                .expect("Unable to parse quantity"),
            from: caps["from"].parse::<usize>().expect("Unable to parse from"),
            to: caps["to"].parse::<usize>().expect("Unable to parse to"),
        }
    }
}


#[derive(Clone, PartialEq, Eq)]
enum CraneModel {
    MODEL9000,
    MODEL9001
}

#[derive(Clone)]
struct DockWork {
    container_stacks: BTreeMap<usize, String>,
    instructions: Vec<Instruction>,
    crane_model: CraneModel
}

impl DockWork {
    fn run(&mut self) {
        for inst in &self.instructions {
            let mut e = String::new();
            let _ = &self.container_stacks
                .entry(inst.from)
                .and_modify(|stack| {
                    let v = stack.len().checked_sub(inst.quantity).expect(format!("Error: unable to compute top stack quantity: {:?}", (stack.len(), inst.quantity)).as_str());
                    e = stack.drain(v..).collect() });
            let _ = &self.container_stacks
                .entry(inst.to)
                .and_modify(|stack| {
                    match self.crane_model {
                        CraneModel::MODEL9000 => stack.push_str((&*e).chars().rev().collect::<String>().as_str()),
                        CraneModel::MODEL9001 => stack.push_str(&*e),

                    }

                });
        }
    }

    // pub(crate)
    fn get_top(&self) -> String {
        let mut out = String::new();
        for (_, val) in &self.container_stacks {
            out.push(val.chars().last().unwrap());
        }
        return out;
    }
}

use std::fmt;
impl fmt::Debug for DockWork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DockWork {{\n {:?}\n {:?}\n}}", self.container_stacks, self.instructions
        )
    }
}

fn parse_fist_input(input: Vec<&str>) -> BTreeMap<usize, String> {
    let mut container_stacks: BTreeMap<usize, String> = BTreeMap::new();
    for s in input {
        let res: Vec<(usize, &str)> = s
            .match_indices(|c: char| c.is_ascii_alphabetic() && !c.is_ascii_whitespace())
            .collect();
        eprintln!("{:?}", res);
        for (id, c) in res {
            container_stacks
                .entry((id/4)+1)
                .and_modify(|stack| stack.insert_str(0, c))
                .or_insert(c.into());
        }
    }
    return container_stacks;
}

fn parse_second_input(input: Vec<&str>) -> Vec<Instruction> {
    let mut instruction_list = Vec::new();
    for s in input {
        instruction_list.push(Instruction::from(s));
    }
    return instruction_list;
}

fn parse(input: String) -> (BTreeMap<usize, String>, Vec<Instruction>)  {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut split = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.trim().is_empty() {
            split = i;
            break;
        }
    }
    let first_input = Vec::from(&lines[0..split]);
    let second_input = Vec::from(&lines[split + 1..]);
    eprintln!("first_input: {:?}", first_input);
    eprintln!("second_input: {:?}", second_input);

    let container_stacks = parse_fist_input(first_input);
    let instruction_list = parse_second_input(second_input);

    (container_stacks, instruction_list)
}

pub fn part_one<S: Into<String>>(input: S) -> String {
    let (container_stacks, instructions) = parse(input.into());
    let mut dockwork = DockWork { container_stacks, instructions, crane_model: CraneModel::MODEL9000 };
    eprintln!("{:?}", dockwork);
    dockwork.run();
    let result = dockwork.get_top();
    result.into()
}

pub fn part_two<S: Into<String>>(input: S) -> String {
    let (container_stacks, instructions) = parse(input.into());
    let mut dockwork = DockWork { container_stacks, instructions, crane_model: CraneModel::MODEL9001 };
    eprintln!("{:?}", dockwork);
    dockwork.run();
    let result = dockwork.get_top();
    result.into()
}

#[cfg(test)]
mod tests {

    use crate::day05;
    const URL: &str = "https://adventofcode.com/2022/day/5/input";

    #[test]
    fn test_01_part_one() {
        let input = crate::util::read_from_input_test_file("day05");

        let result = day05::part_one(input);
        assert_eq!(result, "CMZ")
    }

    #[test]
    // #[ignore = "Not implemented"]
    fn test_02_part_one() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(URL)
            .header(
                "cookie",
                crate::util::get_auth_cookie_session_from_envfile(),
            )
            .send()
            .expect("Unable to get from url");

        let input = response
            .text()
            .expect("Unable to retrieve text from response");
        eprintln!("{}", input);

        let result = day05::part_one(input);
        eprintln!("{}", result);

        assert_eq!(result, "TGWSMRBPN");
        Ok(())
    }

    #[test]
    fn test_03_part_one() {
        let input = crate::util::read_from_input_test_file("day05");

        let result = day05::part_two(input);
        assert_eq!(result, "MCD")
    }

    #[test]
    // #[ignore = "Not implemented"]
    fn test_04_part_two() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(URL)
            .header(
                "cookie",
                crate::util::get_auth_cookie_session_from_envfile(),
            )
            .send()
            .expect("Unable to get from url");

        let input = response
            .text()
            .expect("Unable to retrieve text from response");
        eprintln!("{}", input);

        let result = day05::part_two(input);
        eprintln!("{}", result);

        assert_eq!(result, "TGWSMRBPN");
        Ok(())
    }

}
