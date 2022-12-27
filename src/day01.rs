fn parse(mut input: String) -> Vec<i64> {
    input = input.trim_start().trim_end().to_string();
    let mut elves_packages = Vec::<i64>::new();
    let mut acc = 0;
    let mut lines = input.lines().into_iter();
    let mut has_end = false;
    while !has_end {
        if let Some(line) = lines.next() {
            if line.trim().is_empty() {
                elves_packages.push(acc);
                acc = 0;
            } else {
                acc += line
                .trim()
                .parse::<i64>()
                .expect("Error when parsing elves calories");
            }
        } else {
            elves_packages.push(acc);
            has_end = true;
        }

    }

    // for (i, line) in lines.enumerate() {
    //     if line.trim().is_empty() {
    //         if i <= input.lines().count() - 1 {
    //             eprintln!("{:?}", elves_packages)
    //         }
    //         elves_packages.push(acc);
    //         acc = 0;
    //     } else {
    //         acc += line
    //             .trim()
    //             .parse::<i64>()
    //             .expect("Error when parsing elves calories");
    //     }
    // }
    elves_packages.sort_unstable();
    elves_packages
}

pub fn part_one<S: Into<String>>(input: S) -> i64 {
    let result = parse(input.into());
    let max_calories = result.iter().max().expect("Error while unwrapping max val");
    *max_calories
}

pub fn part_two<S: Into<String>>(input: S) -> i64 {
    let result = parse(input.into());
    // eprintln!("{:?}", result);

    let the_3_max_elves = &result[result.len()-3..result.len()];
    // eprintln!("{:?}", the_3_max_elves);
    let max_3_calories = the_3_max_elves.iter().sum();
    max_3_calories
}


#[cfg(test)]
mod tests {
    use crate::day01;
    const URL: &str = "https://adventofcode.com/2022/day/1/input";


    #[test]
    fn test_01_part_one() {
        let input: &str = "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000";

        let result: i64 = day01::part_one(input);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_02_part_one() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client.get(URL)
        .header("cookie", crate::util::get_auth_cookie_session_from_envfile())
        .send().expect("Unable to get from url");
        // eprintln!("{:#?}", response);

        let input = response.text().expect("Unable to retrieve text from response");
        // eprintln!("{}", input);

        let result = day01::part_one(input);
        eprintln!("{}", result);

        assert_eq!(result, 72070);
        Ok(())
    }

    #[test]
    fn test_03_part_two() {
        let input = "1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
";

        let result = day01::part_two(input);
        assert_eq!(result, 45000);
    }

    #[test]
    fn test_04_part_two() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client.get(URL)
        .header("cookie", crate::util::get_auth_cookie_session_from_envfile())
        .send().expect("Unable to get from url");
        // eprintln!("{:#?}", response);

        let input = response.text().expect("Unable to retrieve text from response");
        eprintln!("{}", input);

        let result = day01::part_two(input);
        eprintln!("{}", result);

        assert_eq!(result, 211805);
        Ok(())
    }
}