use std::fs;

fn count(bases: String) -> Box<[usize; 4]> {
    let mut counts = [0usize; 4];
    bases.trim().chars().for_each(|c| match c {
        'A' => counts[0] += 1,
        'C' => counts[1] += 1,
        'G' => counts[2] += 1,
        'T' => counts[3] += 1,
        _ => panic!("Unrecognized nucleotide {}", c),
    });
    Box::new(counts)
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("rosalind_dna.txt")?;
    let output = *count(input);
    println!("{:?}", output);
    Ok(())
}

#[test]
fn sample() {
    let input =
        "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC".to_string();
    let output = [20, 12, 17, 21];
    assert_eq!(*count(input), output);
}
