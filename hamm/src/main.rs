use std::fs;

fn hamming(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("rosalind_hamm.txt")?;
    let input = input.lines().collect::<Vec<&str>>();
    let output = hamming(input[0], input[1]);
    println!("{:?}", output);
    Ok(())
}

#[test]
fn sample() {
    let input = String::from("GAGCCTACTAACGGGAT\nCATCGTAATGACGGCCT");
    let input = input.lines().collect::<Vec<&str>>();
    let output = hamming(input[0], input[1]);
    assert_eq!(output, 7);
}
