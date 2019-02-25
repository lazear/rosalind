use std::collections::HashMap;
use std::fs;
use std::io;

fn build_codons<P: AsRef<std::path::Path>>(path: P) -> io::Result<HashMap<String, String>> {
    let f = fs::read_to_string(path)?;
    let mut map = HashMap::new();
    for line in f.lines() {
        let mut iter = line.trim().split_whitespace();
        let codon = iter.next();
        let aa = iter.next();
        match (codon, aa) {
            (Some(c), Some(a)) => map.insert(c.to_string(), a.to_string()),
            _ => return Ok(map),
        };
    }
    Ok(map)
}

fn translate(input: &str, map: &HashMap<String, String>) -> String {
    let mut output = String::new();
    for i in (0..input.len() - 3).step_by(3) {
        let s = &input[i..i + 3];
        let aa = map.get(s).unwrap();
        if aa == "X" {
            break;
        }
        output.push_str(aa);
    }
    output
}

fn main() -> io::Result<()> {
    let map = build_codons("codons")?;
    let input = fs::read_to_string("rosalind_prot.txt")?;
    let output = translate(&input, &map);
    println!("{}", output);
    Ok(())
}

#[test]
fn sample() {
    let map = build_codons("codons").unwrap();
    let input = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
    let output = translate(input, &map);
    assert_eq!(&output, "MAMAPRTEINSTRING");
}
