pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;


#[cfg(test)]
mod tests {

    const AUTH_COOKIE: &str = "session=53616c7465645f5f29d87bef140365d31884c705f6a101fe023cbe1fd684d44eb8e4d3b80077285f258bbf96a1d855802d96c0754ee8decfbb1e4dbbea133235";

    mod day01 {
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
            .header("cookie", crate::tests::AUTH_COOKIE)
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
            .header("cookie", crate::tests::AUTH_COOKIE)
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

    mod day02 {
        use crate::day02;
        const URL: &str = "https://adventofcode.com/2022/day/2/input";

        #[test]
        fn test_01_part_one() {
            let input = "A Y
            B X
            C Z";

            let result = day02::part_one(input);

            assert_eq!(result, 15)
        }

        #[test]
        fn test_02_part_one() -> Result<(), Box<dyn std::error::Error>> {
            let client = reqwest::blocking::Client::new();
            let response = client.get(URL)
            .header("cookie", crate::tests::AUTH_COOKIE)
            .send().expect("Unable to get from url");

            let input = response.text().expect("Unable to retrieve text from response");
            eprintln!("{}", input);

            let result = day02::part_one(input);
            eprintln!("{}", result);

            assert_eq!(result, 11475);
            Ok(())
        }

        #[test]
        fn test_03_part_two() {
            let input = "A Y
            B X
            C Z";

            let result = day02::part_two(input);

            assert_eq!(result, 12)
        }

        #[test]
        fn test_04_part_two() -> Result<(), Box<dyn std::error::Error>> {
            let client = reqwest::blocking::Client::new();
            let response = client.get(URL)
            .header("cookie", crate::tests::AUTH_COOKIE)
            .send().expect("Unable to get from url");

            let input = response.text().expect("Unable to retrieve text from response");
            eprintln!("{}", input);

            let result = day02::part_two(input);
            eprintln!("{}", result);

            assert_eq!(result, 16862);
            Ok(())
        }
    }

    mod day03 {
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
            .header("cookie", crate::tests::AUTH_COOKIE)
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
            .header("cookie", crate::tests::AUTH_COOKIE)
            .send().expect("Unable to get from url");

            let input = response.text().expect("Unable to retrieve text from response");
            eprintln!("{}", input);

            let result = day03::part_two(input);
            eprintln!("{}", result);

            assert_eq!(result, 2342);
            Ok(())
        }

    }

    mod day04 {
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
            .header("cookie", crate::tests::AUTH_COOKIE)
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
            .header("cookie", crate::tests::AUTH_COOKIE)
            .send().expect("Unable to get from url");

            let input = response.text().expect("Unable to retrieve text from response");
            eprintln!("{}", input);

            let result = day04::part_two(input);
            eprintln!("{}", result);

            assert_eq!(result, 0);
            Ok(())
        }
    }

}
