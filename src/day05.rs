use std::collections::BTreeMap;

#[derive(Debug)]
struct Instruction {

}

fn parse(mut input: String) -> (BTreeMap<usize, Vec<char>>, Vec<Instruction>){
    let mut container_stacks = BTreeMap::new();
    let mut instruction_list = Vec::new();
    (container_stacks, instruction_list)
}

pub fn part_one<S: Into<String>>(input: S) -> String {
    let data = parse(input.into());
    eprintln!("{:?}", data);
    let result = "";
    result.into()
}

pub fn part_two<S: Into<String>>(input: S) -> String {
    let data = parse(input.into());
    eprintln!("{:?}", data);
    let result = "";
    result.into()
}

#[cfg(test)]
mod tests {
    use crate::day05;
    const URL: &str = "https://adventofcode.com/2022/day/5/input";

    #[test]
    fn test_01_part_one() {
        let input = "
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2";

        let result = day05::part_one(input);

        assert_eq!(result, "")
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

        let result = day05::part_one(input);
        eprintln!("{}", result);

        assert_eq!(result, "");
        Ok(())
    }
}