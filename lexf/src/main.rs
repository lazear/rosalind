use std::fs::{self};
use std::io::Write;

fn lexi_kmers(
  start: usize,
  n: usize,
  slice: &mut [char],
  alphabet: &Vec<char>,
  out: &mut Vec<Vec<char>>,
) {
  if start == n {
    out.push(Vec::from(slice));
    return;
  }

  for i in 0..alphabet.len() {
    slice[start] = alphabet[i];
    lexi_kmers(start + 1, n, slice, alphabet, out);
  }
}

fn main() -> std::io::Result<()> {
  let mut out = Vec::<Vec<char>>::new();
  let s = std::fs::read_to_string("rosalind_lexf.txt")?;
  let alphabet = s
    .lines()
    .nth(0)
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.chars().next().unwrap())
    .collect::<Vec<char>>();
  let seq_len = s.lines().nth(1).unwrap().trim().parse::<usize>().unwrap();
  let mut slice = vec![alphabet[0]; seq_len];
  lexi_kmers(0, seq_len, &mut slice, &alphabet, &mut out);
  let mut f = fs::File::create("out.txt")?;
  out
    .into_iter()
    .for_each(|v| writeln!(f, "{}", v.into_iter().collect::<String>()).unwrap());

  Ok(())
}

#[test]
fn sample() {
  let out = vec![
    vec!['A', 'A'],
    vec!['A', 'B'],
    vec!['B', 'A'],
    vec!['B', 'B'],
  ];
  let mut out2 = Vec::<Vec<char>>::new();
  let alphabet = vec!['A', 'B'];
  let seq_len = 2;
  let mut slice = vec![alphabet[0]; seq_len];
  lexi_kmers(0, seq_len, &mut slice, &alphabet, &mut out2);
  assert_eq!(out, out2);
}
