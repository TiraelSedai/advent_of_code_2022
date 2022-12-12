pub(crate) fn solve(input: impl Iterator<Item = String>) {
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

    for line in (0..240).collect::<Vec<usize>>().chunks_exact(40) {
        for pixel in line {
            let sprite_pos = states[*pixel];
            if (sprite_pos-1..=(sprite_pos+1)).contains(&((*pixel as i32) % 40)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
}