pub(crate) fn solve(input: impl Iterator<Item = String>) {
    let input = input
        .map(|line| {
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
        let (first, last) = (&line[0], &line[1]);
        if (first[0] <= last[0] && first[1] >= last[1])
            || (first[0] >= last[0] && first[1] <= last[1])
        {
            acc + 1
        } else {
            acc
        }
    });
        
    println!("{}", result)
}
