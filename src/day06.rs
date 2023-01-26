fn parse(input: String) {
    let lines = input.lines().collect::<Vec<&str>>();
}

pub fn part_one<S: Into<String>>(input: S) -> usize {
    let data:String = input.into();
    let mut occurences = String::new();
    for (i, c) in data.chars().enumerate() {
        if i > 3 && occurences.chars(). {

        }
    }
    0
}

pub fn part_two<S: Into<String>>(input: S) -> usize {

    0
}

#[cfg(test)]
mod tests {

    use crate::day06;
    const URL: &str = "https://adventofcode.com/2022/day/5/input";

    #[test]
    fn test_01_part_one() {
        let inputs = [
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
        ];

        assert_eq!(day06::part_one(inputs[0]), 5);
        assert_eq!(day06::part_one(inputs[1]), 6);
        assert_eq!(day06::part_one(inputs[2]), 10);
        assert_eq!(day06::part_one(inputs[3]), 11);
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

        let result = day06::part_one(input);
        eprintln!("{}", result);

        assert_eq!(result, "TGWSMRBPN");
        Ok(())
    }

    #[test]
    fn test_03_part_one() {
        let input = crate::util::read_from_input_test_file("day05");

        let result = day06::part_two(input);
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

        let result = day06::part_two(input);
        eprintln!("{}", result);

        assert_eq!(result, "TGWSMRBPN");
        Ok(())
    }
}
