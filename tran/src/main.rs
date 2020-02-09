use utils::{Fasta, Nucleotide};

fn tran(dna1: &String, dna2: &String) -> String {
    let dna1_vec = dna1
        .chars()
        .map(Nucleotide::from_char)
        .collect::<Vec<Nucleotide>>();
    let dna2_vec = dna2
        .chars()
        .map(Nucleotide::from_char)
        .collect::<Vec<Nucleotide>>();

    let mut transition = 0;
    let mut transversion = 0;

    for i in 0..dna1_vec.len() {
        if dna1_vec[i] == dna2_vec[i] {
            continue;
        }
        match (dna1_vec[i], dna2_vec[i]) {
            (Nucleotide::A, Nucleotide::G) => transition += 1,
            (Nucleotide::G, Nucleotide::A) => transition += 1,
            (Nucleotide::C, Nucleotide::T) => transition += 1,
            (Nucleotide::T, Nucleotide::C) => transition += 1,
            _ => transversion += 1,
        }
    }

    format!("{:.11}", transition as f64 / transversion as f64)
}

fn main() -> std::io::Result<()> {
    let input = Fasta::parse_file_vec("rosalind_tran.txt")?;
    let seq1 = &input[0].1;
    let seq2 = &input[1].1;
    println!("{}", tran(seq1, seq2));
    Ok(())
}

#[test]
fn sample() {
    let seq1 = String::from(
        "GCAACGCACAACGAAAACCCTTAGGGACTGGATTATTTCGTGATCGTTGTAGTTATTGGAAGTACGGGCATCAACCCAGTT",
    );
    let seq2 = String::from(
        "TTATCTGACAAAGAAAGCCGTCAACGGCTGGATAATTTCGCGATCGTGCTGGTTACTGGCGGTACGAGTGTTCCTTTGGGT",
    );

    assert_eq!("1.21428571429", tran(&seq1, &seq2));
}
