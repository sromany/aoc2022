pub mod day01;
pub mod day02;


#[cfg(test)]
mod tests {

    mod day01 {
        use crate::day01;
        #[test]
        fn test_01_part_one() {
            let input = "
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

            let result = day01::part_one(input);
            assert_eq!(result, 24000);
        }

        #[test]
        fn test_02_part_one() -> Result<(), Box<dyn std::error::Error>> {
            let client = reqwest::blocking::Client::new();
            let response = client.get("https://adventofcode.com/2022/day/1/input")
            .header("cookie", "session=53616c7465645f5f29d87bef140365d31884c705f6a101fe023cbe1fd684d44eb8e4d3b80077285f258bbf96a1d855802d96c0754ee8decfbb1e4dbbea133235")
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
            let response = client.get("https://adventofcode.com/2022/day/1/input")
            .header("cookie", "session=53616c7465645f5f29d87bef140365d31884c705f6a101fe023cbe1fd684d44eb8e4d3b80077285f258bbf96a1d855802d96c0754ee8decfbb1e4dbbea133235")
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
            let response = client.get("https://adventofcode.com/2022/day/2/input")
            .header("cookie", "session=53616c7465645f5f29d87bef140365d31884c705f6a101fe023cbe1fd684d44eb8e4d3b80077285f258bbf96a1d855802d96c0754ee8decfbb1e4dbbea133235")
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
            let response = client.get("https://adventofcode.com/2022/day/2/input")
            .header("cookie", "session=53616c7465645f5f29d87bef140365d31884c705f6a101fe023cbe1fd684d44eb8e4d3b80077285f258bbf96a1d855802d96c0754ee8decfbb1e4dbbea133235")
            .send().expect("Unable to get from url");

            let input = response.text().expect("Unable to retrieve text from response");
            eprintln!("{}", input);

            let result = day02::part_two(input);
            eprintln!("{}", result);

            assert_eq!(result, 16862);
            Ok(())
        }
    }
}
