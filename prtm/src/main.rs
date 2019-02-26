use std::fs;
use utils::Monoisotopic;

fn main() -> std::io::Result<()> {
    let table = utils::Monoisotopic::new();
    let f = fs::read_to_string("rosalind_prtm.txt")?;
    println!("{}", table.peptide_mass(f.trim()));
    Ok(())
}

#[test]
fn sample() {
    let input = "SKADYEK";
    let e = 0.001;
    assert!((Monoisotopic::new().peptide_mass(input) - 821.392).abs() <= e);
}
