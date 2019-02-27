use std::collections::HashSet;
use utils::{CodonTable, Fasta, Nucleotide, Sequence};

fn open_reading_frames(ct: &CodonTable, s: &str) -> HashSet<String> {
    let mut orfs = HashSet::new();
    for i in 0..s.len() {
        if let Some(orf) = ct.translate(&s[i..]) {
            if orf.starts_with("M") && orf.contains("X") {
                orfs.insert(orf.chars().take_while(|&ch| ch != 'X').collect::<String>());
            }
        }
    }
    orfs
}

fn problem(seq: &str) -> HashSet<String> {
    let ct = CodonTable::default();

    let fwd = Sequence::new(seq)
        .nucleotides()
        .into_iter()
        .map(Nucleotide::transcribe)
        .map(Nucleotide::to_char)
        .collect::<String>();
    let rev = Sequence::new(seq)
        .reverse_complement()
        .into_iter()
        .map(Nucleotide::transcribe)
        .map(Nucleotide::to_char)
        .collect::<String>();

    let f = open_reading_frames(&ct, &fwd);
    let r = open_reading_frames(&ct, &rev);
    f.union(&r).cloned().collect()
}

fn main() -> std::io::Result<()> {
    let input = Fasta::parse_file("rosalind_orf.txt")?;
    let seq = input.values().next().unwrap();

    for orf in problem(seq) {
        println!("{}", orf);
    }

    Ok(())
}

#[test]
fn sample() {
    let input = Fasta::parse_file("sample.txt").unwrap();
    let seq = input.values().next().unwrap();
    let answer = vec![
        "MLLGSFRLIPKETLIQVAGSSPCNLS",
        "M",
        "MGMTPRLGLESLLE",
        "MTPRLGLESLLE",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect::<HashSet<_>>();

    assert_eq!(problem(seq), answer);
}
