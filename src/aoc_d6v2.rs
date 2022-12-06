use std::collections::HashSet;

pub(crate) fn solve(input: impl Iterator<Item = String>) {
    let input = input.collect::<Vec<String>>();
    let input = input[0].chars().collect::<Vec<char>>();
    let len = 14;
    for i in 0..usize::MAX {
        let start = i;
        let end = i + len;
        let slice = &input[start..end];

        // would be easy with Itertools :: unique, but let's do it by hand
        let mut set = HashSet::with_capacity(len);
        for c in slice {
            if set.contains(c) {
                break;
            } else {
                set.insert(c);
            }
        }
        if set.len() == len {
            println!("{}", end);
            return;
        }
    }
}
