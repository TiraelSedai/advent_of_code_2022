use std::collections::HashSet;

pub(crate) fn solve(input: impl Iterator<Item = String>) {
    let mut result = 0_u32;
    for line in input{
        let half_len = line.len() / 2;
        let first: HashSet<char> = line[..half_len].chars().collect();
        let last: HashSet<char> = line[half_len..].chars().collect();
        let same = first.intersection(&last).next().unwrap();
        let as_ascii = *same as u32;
        let priority = if as_ascii > 97 {
            as_ascii - 96
        } else {
            26 + as_ascii - 64
        };
        result += priority;
    }

    println!("{result}")
}
