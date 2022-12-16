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
