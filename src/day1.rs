fn process_input(input: Vec<String>) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();

    input.iter().for_each(|val| {
        values.push(val.parse::<i32>().unwrap());
    });

    return values;
}

pub fn part1(input: Vec<String>) -> i32 {
    let values = process_input(input);

    for a in 0..values.len() {
        for b in 0..values.len() {
            if values[a] + values[b] == 2020 {
                return values[a] * values[b];
            }
        }
    }

    return 0;
}

pub fn part2(input: Vec<String>) -> i32 {
    let values = process_input(input);

    for a in 0..values.len() {
        for b in 0..values.len() {
            for c in 0..values.len() {
                if values[a] + values[b] + values[c] == 2020 {
                    return values[a] * values[b] * values[c]
                }
            }
        }
    }

    return 0;
}