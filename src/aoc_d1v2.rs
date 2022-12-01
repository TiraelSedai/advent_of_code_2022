pub(crate) fn solve(input: impl Iterator<Item = String>) {
    let input: Vec<String> = input.collect();
    let mut elves_load = input
        .split(|line| line.is_empty())
        .map(|elf| elf.iter().map(|i| i.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<i32>>();
    elves_load.sort();
    let result: i32 = elves_load.iter().rev().take(3).sum();

    println!("{result}")
}
