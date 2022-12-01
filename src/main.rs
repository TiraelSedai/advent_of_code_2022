use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
mod aoc_d1v2;

fn main() {
    let lines = read_lines(Path::new("input\\1.txt"));
    aoc_d1v2::solve(lines);
}

fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Input not found");
    let lines = BufReader::new(file).lines();
    let text = lines.map(|x| x.unwrap());
    text
}