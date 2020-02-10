use std::collections::HashMap;
use std::fs;
use std::io;

fn build_aa_codons<P: AsRef<std::path::Path>>(path: P) -> io::Result<HashMap<char, Vec<String>>> {
  let f = fs::read_to_string(path)?;
  let mut map: HashMap<char, Vec<String>> = HashMap::new();
  for line in f.lines() {
    let mut iter = line.trim().split_whitespace();
    let codon = iter.next();
    let aa = iter.next();

    match (codon, aa) {
      (Some(c), Some(a)) => map
        .entry(a.chars().next().unwrap())
        .or_insert(Vec::new())
        .push(c.to_string()),
      _ => return Ok(map),
    };
  }
  Ok(map)
}

fn calc_rna_count(input: &str, map: &HashMap<char, Vec<String>>) -> u128 {
  let mut output: u128 = 3; // 3 for STOP codons
  for ch in input.chars() {
    output *= map.get(&ch).unwrap().len() as u128;
  }
  output
}

fn main() -> io::Result<()> {
  let map = build_aa_codons("codons")?;
  let input = fs::read_to_string("rosalind_mrna.txt")?;
  let output = calc_rna_count(&input, &map) % 1_000_0000;
  println!("{}", output);
  Ok(())
}

#[test]
fn sample() {
  let map = build_aa_codons("codons").unwrap();
  let input = "MAA";
  let output = calc_rna_count(input, &map) % 1_000_0000;
  assert_eq!(output, 48);
}
