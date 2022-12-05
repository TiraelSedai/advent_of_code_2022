pub(crate) fn solve(input: impl Iterator<Item = String>) {
    let stack1 = vec!["B", "V", "S", "N", "T", "C", "H", "Q"];
    let stack2 = vec!["W", "D", "B", "G"];
    let stack3 = vec!["F", "W", "R", "T", "S", "Q", "B"];
    let stack4 = vec!["L", "G", "W", "S", "Z", "J", "D", "N"];
    let stack5 = vec!["M", "P", "D", "V", "F"];
    let stack6 = vec!["F", "W", "J"];
    let stack7 = vec!["L", "N", "Q", "B", "J", "V"];
    let stack8 = vec!["G", "T", "R", "C", "J", "Q", "S", "N"];
    let stack9 = vec!["J", "S", "Q", "C", "W", "D", "M"];

    let mut stack = vec![
        stack1, stack2, stack3, stack4, stack5, stack6, stack7, stack8, stack9,
    ];

    //let actions: Vec<String> = input.collect();
    let asd: Vec<Vec<usize>> = input
        .map(|x| {
            x.split_whitespace()
                .enumerate()
                .filter(|(i, _)| i % 2 == 1)
                .map(|(_, e)| e.parse::<usize>().expect("Unable to parse move command part"))
                .collect::<Vec<usize>>()
        })
        .collect();

    for mov in asd {
        let amt = mov[0];
        let src = mov[1] - 1;
        let dst = mov[2] - 1;

        for _ in 0..amt {
            let temp = stack[src].pop().expect("Not enough crates in stack");
            stack[dst].push(temp);
        }
    }

    for i in stack {
        println!("{}", i.last().unwrap());
    }
}
