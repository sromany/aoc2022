fn parse(mut input: String) -> Vec<i64> {

    input = input.trim_start().trim_end().to_string();
    let mut elves_packages = Vec::<i64>::new();
    let mut acc = 0;

    for line in input.lines() {
        if line.trim().len() == 0 {
            elves_packages.push(acc);
            acc = 0;
            continue;
        }
        acc += line.trim().parse::<i64>().expect("Error when parsing elves calories");
    }
    elves_packages
}

pub fn part_one<S: Into<String>>(input: S) -> i64 {

    let result = parse(input.into());
    let max_calories = result.iter().max().unwrap();
    *max_calories
}
