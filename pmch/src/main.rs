use utils::{Fasta, Nucleotide};

fn factorial(num: u64) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

fn pmch(seq: &String) -> u64 {
    let seq_vec = seq
        .chars()
        .map(Nucleotide::from_char)
        .collect::<Vec<Nucleotide>>();
    let mut au_count = 0;
    let mut gs_count = 0;
    for i in 0..seq_vec.len() {
        match seq_vec[i] {
            //assume a&u g&s counts equal
            Nucleotide::A => au_count += 1,
            Nucleotide::G => gs_count += 1,
            _ => (),
        }
    }
    factorial(au_count) * factorial(gs_count)
}
fn main() -> std::io::Result<()> {
    let input = Fasta::parse_file_vec("rosalind_pmch.txt")?;
    let seq1 = &input[0].1;
    println!("{}", pmch(seq1));
    Ok(())
}

#[test]
fn sample() {
    let seq1 = String::from("AGCUAGUCAU");
    assert_eq!(12, pmch(&seq1));
}
