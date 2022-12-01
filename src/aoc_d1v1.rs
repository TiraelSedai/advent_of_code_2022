pub(crate) fn solve(input: impl Iterator<Item = String>) {
    let input: Vec<String> = input.collect();
    let result = input
        .split(|line| line.is_empty())
        .map(|elf| {
            elf.iter()
                .map(|i| i.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap();

    println!("{result}")
}
