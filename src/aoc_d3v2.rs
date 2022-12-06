use std::collections::HashSet;

pub(crate) fn solve(input: impl Iterator<Item = String>) {
    let input: Vec<String> = input.collect();
    let mut result: u32 = 0;
    for group in input.chunks_exact(3) {
        let first: HashSet<char> = group[0].chars().collect();
        let second: HashSet<char> = group[1].chars().collect();
        let third: HashSet<char> = group[2].chars().collect();
        let common = first.intersection(&second).map(|x| *x).collect::<HashSet<char>>();
        let same = common.intersection(&third).next().unwrap();

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
