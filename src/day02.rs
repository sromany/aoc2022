use core::panic;

#[derive(Clone, Copy)]
enum Janken {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Janken {
    fn from(c: char) -> Janken {
        match c {
            'A' | 'X' => Janken::Rock,
            'B' | 'Y' => Janken::Paper,
            'C' | 'Z' => Janken::Scissors,
            _ => panic!("Unknown char {:?}", c),
        }
    }
}

impl Janken {
    fn value(self) -> isize {
        match self {
            Janken::Rock => 1,
            Janken::Paper => 2,
            Janken::Scissors => 3,
        }
    }

    fn score(opponent: Janken, player: Janken) -> isize {
        (match (opponent, player) {
            (Janken::Rock, Janken::Rock) => 3,
            (Janken::Rock, Janken::Paper) => 6,
            (Janken::Rock, Janken::Scissors) => 0,
            (Janken::Paper, Janken::Rock) => 0,
            (Janken::Paper, Janken::Paper) => 3,
            (Janken::Paper, Janken::Scissors) => 6,
            (Janken::Scissors, Janken::Rock) => 6,
            (Janken::Scissors, Janken::Paper) => 0,
            (Janken::Scissors, Janken::Scissors) => 3,
        }) + player.value()
    }

    fn win_against(hand: Janken) -> Janken {
        match hand {
            Janken::Rock => Janken::Paper,
            Janken::Paper => Janken::Scissors,
            Janken::Scissors => Janken::Rock,
        }
    }

    fn lose_against(hand: Janken) -> Janken {
        match hand {
            Janken::Rock => Janken::Scissors,
            Janken::Paper => Janken::Rock,
            Janken::Scissors => Janken::Paper,
        }
    }
}

enum Outcome {
    Draw,
    Win,
    Lose,
}

impl From<char> for Outcome {
    fn from(_c: char) -> Self {
        match _c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Unknown char {:?}", _c),
        }
    }
}

fn parse<T1: From<char>, T2: From<char>>(input: String) -> Vec<(T1, T2)> {
    let mut lines = input.lines();
    let mut janken_games: Vec<(T1, T2)> = Vec::<(T1, T2)>::new();
    let mut has_end: bool = false;
    while !has_end {
        if let Some(line) = lines.next() {
            eprintln!("{:?}", line.trim());
            let left = T1::from(
                line.trim()
                    .chars()
                    .nth(0)
                    .expect("Error when parse oponent sign"),
            );
            let right = T2::from(
                line.trim()
                    .chars()
                    .nth(2)
                    .expect("Error when parse my sign"),
            );
            janken_games.push((left, right))
        } else {
            has_end = true;
        }
    }
    return janken_games;
}

// =============== Part One =========================================================
pub fn part_one<S: Into<String>>(input: S) -> isize {
    let games = parse::<Janken, Janken>(input.into());
    compute_winning_score_from_games(games)
}

fn compute_winning_score_from_games(games: Vec<(Janken, Janken)>) -> isize {
    let mut result = 0;
    for (opponent, player) in games {
        result += Janken::score(opponent, player);
    }
    return result;
}
// ==================================================================================

// =============== Part Two =========================================================
pub fn part_two<S: Into<String>>(input: S) -> isize {
    let strategy_guide = parse::<Janken, Outcome>(input.into());
    let games = create_games_from_strategy_guide(strategy_guide);
    compute_winning_score_from_games(games)
}

fn create_games_from_strategy_guide(
    strategy_plan: Vec<(Janken, Outcome)>,
) -> Vec<(Janken, Janken)> {
    let mut games: Vec<(Janken, Janken)> = Vec::new();
    for (opponent, outcome) in strategy_plan {
        let player_play = match outcome {
            Outcome::Draw => opponent,
            Outcome::Win => Janken::win_against(opponent),
            Outcome::Lose => Janken::lose_against(opponent),
        };
        games.push((opponent, player_play));
    }
    return games;
}
// ==================================================================================

#[cfg(test)]
mod tests {
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
        .header("cookie", crate::util::get_auth_cookie_session_from_envfile())
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
        .header("cookie", crate::util::get_auth_cookie_session_from_envfile())
        .send().expect("Unable to get from url");

        let input = response.text().expect("Unable to retrieve text from response");
        eprintln!("{}", input);

        let result = day02::part_two(input);
        eprintln!("{}", result);

        assert_eq!(result, 16862);
        Ok(())
    }
}