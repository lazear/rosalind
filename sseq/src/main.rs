use utils::{Fasta, Nucleotide};

fn sseq(seq: &String, sub_seq: &String) -> Vec<usize> {
    let seqv = seq
        .chars()
        .map(Nucleotide::from_char)
        .collect::<Vec<Nucleotide>>();
    let sub_seqv = sub_seq
        .chars()
        .map(Nucleotide::from_char)
        .collect::<Vec<Nucleotide>>();
    let mut res = vec![0; sub_seqv.len()];
    let mut resi = 0;
    let mut cur = sub_seqv[0];

    for i in 0..seqv.len() {
        if seqv[i] == cur {
            res[resi] = i + 1;
            resi += 1;
            if resi == sub_seqv.len() {
                break;
            }
            cur = sub_seqv[resi];
        }
    }
    res
}

fn main() -> std::io::Result<()> {
    let input = Fasta::parse_file_vec("rosalind_sseq.txt")?;
    let seq = &input[0].1;
    let sub_seq = &input[1].1;
    println!("{:?}", sseq(seq, sub_seq));
    Ok(())
}

#[test]
fn sample() {
    let seq1 = String::from("ACGTACGTGACG");
    let seq2 = String::from("CAC");

    assert_eq!(vec![2, 5, 6].to_vec(), sseq(&seq1, &seq2));
}
