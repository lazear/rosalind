use std::fs::{self};

fn pper(n: usize, k: usize) -> usize {
    let mut res = 1;
    (n - k + 1..=n).for_each(|v| res = res * v % 1_000_000);
    res
}
fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("rosalind_pper.txt")?
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("{}", pper(input[0], input[1]));
    Ok(())
}

#[test]
fn sample() {
    assert_eq!(51200, pper(21, 7));
}
