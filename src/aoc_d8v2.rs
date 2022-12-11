use std::ops::Range;

pub(crate) fn solve(input: impl Iterator<Item = String>) {
    let mut max = 0_u32;
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
            let score = visible_score(&input, column, row, tall, wide);
            if score > max {
                max = score;
            }
        }
    }

    println!("{max}")
}

fn visible_score(input: &[Vec<u32>], column: usize, row: usize, tall: usize, wide: usize) -> u32 {
    if column == 0 || column == wide - 1 || row == 0 || row == tall - 1 {
        return 0;
    }
    let cell = input[row][column];

    visible_score_side(input, cell, 0..row, column..column + 1, true)
        * visible_score_side(input, cell, row + 1..tall, column..column + 1, false)
        * visible_score_side(input, cell, row..row + 1, 0..column, true)
        * visible_score_side(input, cell, row..row + 1, column + 1..wide, false)
}

fn visible_score_side(
    input: &[Vec<u32>],
    cell: u32,
    vertical: Range<usize>,
    horizontal: Range<usize>,
    revert: bool,
) -> u32 {
    let mut result = 0;

    if !revert {
        for i in vertical {
            for j in horizontal.clone() {
                result += 1;
                if input[i][j] >= cell {
                    return result;
                }
            }
        }
    } else {
        for i in vertical.rev() {
            for j in horizontal.clone().rev() {
                result += 1;
                if input[i][j] >= cell {
                    return result;
                }
            }
        }
    }
    result
}
