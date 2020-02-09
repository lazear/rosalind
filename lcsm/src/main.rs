use utils::Fasta;

fn find_motif(input: Vec<&str>) -> &str {
  // find smallest dna string for reference
  let mut ref_dna_index = 0;
  let mut ref_len = 1000;
  for (i, item) in input.iter().enumerate() {
    if item.len() < ref_len {
      ref_len = item.len();
      ref_dna_index = i;
    }
  }

  // find shared motif
  let mut motif_len = ref_len; //inital start
  loop {
    let mut i = 0;
    while i + motif_len <= ref_len && motif_len > 1 {
      let mut motif = &input[ref_dna_index][i..motif_len + i];
      let mut found = true;
      for j in 0..input.len() {
        if j == ref_dna_index {
          continue;
        }
        if !input[j].contains(motif) {
          motif = "";
          found = false;
          break;
        }
      }
      if found {
        return motif;
      }
      i = i + 1;
    }
    motif_len = motif_len - 1;
    if motif_len == 1 {
      return "";
    }
  }
}

fn main() -> std::io::Result<()> {
  let input = Fasta::parse_file("rosalind_lcsm.txt")?;
  println!("{}", find_motif(input.values().map(|s| s.trim()).collect()));
  Ok(())
}

#[test]
fn sample() {
  assert_eq!(
    "ATAG",
    find_motif(vec!["UUATAGC", "CCATAGU", "TTATAGTT", "ATAGGGGGGGGGG"])
  )
}
