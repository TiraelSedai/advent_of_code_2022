struct Monkey {
    items: Vec<u64>,
    operation: fn(u64) -> u64,
    test: fn(u64) -> bool,
    true_index: usize,
    false_index: usize,
}

pub(crate) fn solve(_input: impl Iterator<Item = String>) {
    let mut monkeys = fill_input();
    let mut operations = vec![0; 8];

    let magic = 11 * 17 * 5 * 13 * 19 * 2 * 3 * 7;

    for _step in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            let monkeys = &mut monkeys;
            for _ in 0..monkeys[monkey_index].items.len() {
                let curr_monkey = &mut monkeys[monkey_index];
                let item = curr_monkey.items.pop().unwrap();
                operations[monkey_index] += 1;
                let item = (curr_monkey.operation)(item) % magic;
                let dest_index = if (curr_monkey.test)(item) {
                    curr_monkey.true_index
                } else {
                    curr_monkey.false_index
                };
                monkeys[dest_index].items.push(item);
            }
        }
    }

    operations.sort();
    println!("{:?}", operations);
}

fn fill_input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![98, 97, 98, 55, 56, 72],
            operation: |x| x * 13,
            test: |x| x % 11 == 0,
            true_index: 4,
            false_index: 7,
        },
        Monkey {
            items: vec![73, 99, 55, 54, 88, 50, 55],
            operation: |x| x + 4,
            test: |x| x % 17 == 0,
            true_index: 2,
            false_index: 6,
        },
        Monkey {
            items: vec![67, 98],
            operation: |x| x * 11,
            test: |x| x % 5 == 0,
            true_index: 6,
            false_index: 5,
        },
        Monkey {
            items: vec![82, 91, 92, 53, 99],
            operation: |x| x + 8,
            test: |x| x % 13 == 0,
            true_index: 1,
            false_index: 2,
        },
        Monkey {
            items: vec![52, 62, 94, 96, 52, 87, 53, 60],
            operation: |x| x * x,
            test: |x| x % 19 == 0,
            true_index: 3,
            false_index: 1,
        },
        Monkey {
            items: vec![94, 80, 84, 79],
            operation: |x| x + 5,
            test: |x| x % 2 == 0,
            true_index: 7,
            false_index: 0,
        },
        Monkey {
            items: vec![89],
            operation: |x| x + 1,
            test: |x| x % 3 == 0,
            true_index: 0,
            false_index: 5,
        },
        Monkey {
            items: vec![70, 59, 63],
            operation: |x| x + 3,
            test: |x| x % 7 == 0,
            true_index: 4,
            false_index: 3,
        },
    ]
}
