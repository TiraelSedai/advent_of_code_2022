pub(crate) fn solve(input: impl Iterator<Item = String>) {
    //test();
    let mut state = 1;
    let mut states: Vec<i32> = Vec::new();

    for line in input {
        states.push(state);
        if line.starts_with("addx") {
            let add = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            states.push(state);
            state += add;
        }
    }

    let mut result = 0;
    for cycle in (20_usize..=220).step_by(40) {
        result += states[cycle - 1] * (cycle as i32);
    }
    println!("{result}")
}

fn test() {
    let input = include_str!("..\\input\\10_test.txt").lines();

    let mut state = 1;
    let mut states: Vec<i32> = Vec::new();

    for line in input {
        if line == "noop" {
            states.push(state);
        } else if line.starts_with("addx") {
            let add = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            states.push(state);
            states.push(state);
            state += add;
        } else {
            unreachable!();
        }
    }

    for cycle in (20_usize..=220).step_by(40) {
        println!("{}", states[cycle - 1] * (cycle as i32));
    }
}
