use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
mod aoc_d11v1;

fn main() {
    let lines = read_lines(Path::new("input\\11.txt"));
    aoc_d11v1::solve(lines);
}

fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Input not found");
    let lines = BufReader::new(file).lines();
    lines.map(|x| x.unwrap())
}
