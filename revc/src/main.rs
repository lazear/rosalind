use std::fs;

fn complement(ch: char) -> char {
    match ch {
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => panic!("Unrecognized nucelotide: {}", ch),
    }
}

fn revc(s: String) -> String {
    s.trim().chars().rev().map(complement).collect()
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("rosalind_revc.txt")?;
    let output = revc(input);
    println!("{:?}", output);
    Ok(())
}

#[test]
fn sample() {
    let input = "AAAACCCGGT".to_string();
    let output = revc(input);
    assert_eq!(&output, "ACCGGGTTTT");
}
