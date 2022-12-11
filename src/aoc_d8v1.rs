use std::ops::Range;

pub(crate) fn solve(input: impl Iterator<Item = String>) {
    let mut result = 0_u32;
    let input = input
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let tall = input.len();
    let wide = input[0].len();

    for row in 0..tall {
        for column in 0..wide {
            if visible(&input, column, row, tall, wide) {
                result += 1;
            }
        }
    }

    println!("{result}")
}

fn visible(input: &[Vec<u32>], column: usize, row: usize, tall: usize, wide: usize) -> bool {
    if column == 0 || column == wide - 1 || row == 0 || row == tall - 1 {
        return true;
    }
    let cell = input[row][column];
    visible_one_side(input, cell, 0..row, column..column + 1)
        || visible_one_side(input, cell, row + 1..tall, column..column + 1)
        || visible_one_side(input, cell, row..row + 1, 0..column)
        || visible_one_side(input, cell, row..row + 1, column + 1..wide)
}

fn visible_one_side(
    input: &[Vec<u32>],
    cell: u32,
    vertical: Range<usize>,
    horizontal: Range<usize>,
) -> bool {
    for i in vertical {
        for j in horizontal.clone() {
            if input[i][j] >= cell {
                return false;
            }
        }
    }
    true
}
