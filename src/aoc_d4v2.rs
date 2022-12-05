pub(crate) fn solve(input: impl Iterator<Item = String>) {
    let input = input.map(|line| {
        line.split(',')
            .map(|section| {
                section
                    .split('-')
                    .map(|x| x.parse::<usize>().expect("Cannot parse number"))
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>()
    });

    let result = input.fold(0_usize, |acc, line| {
        let (first, last) = if line[0][0] < line[1][0] {
            (&line[0], &line[1])
        } else {
            (&line[1], &line[0])
        };

        if first[1] >= last[0] {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", result)
}
