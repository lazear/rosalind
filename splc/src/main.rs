use utils::{Fasta, CodonTable};

fn problem(gene: &str, exons: Vec<&str>) -> String {
    let mut indices = Vec::new();
    for ex in exons {
        indices.extend(gene.match_indices(ex).map(|(idx, _)| (idx, idx+ex.len())));
    }
    indices.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out = String::new();
    let mut last = 0;
    for (start, end) in indices {
        let ex = &gene[last..start];
        out.push_str(ex);
        last = end;
    }
    out.push_str(&gene[last..]);
    out.replace("T", "U")
}

fn main() -> std::io::Result<()> {
    let s = std::fs::read_to_string("rosalind_splc.txt")?;
    let id = s.lines().next().unwrap().trim_start_matches(">");
    let fasta = Fasta::parse_string(&s);
    let gene = fasta.get(id).unwrap();
    let exons = fasta.iter().filter(|(k, _)| *k != id).map(|(_, v)| v.as_ref()).collect::<Vec<&str>>();

    let rna = problem(gene, exons);
    let prot = CodonTable::default().translate(&rna).unwrap();
    println!("{}", prot);

    Ok(())
}


#[test]
fn sample() {
    let s = std::fs::read_to_string("sample.txt").unwrap();
    let id = s.lines().next().unwrap().trim_start_matches(">");
    let fasta = Fasta::parse_string(&s);
    let gene = fasta.get(id).unwrap();
    let exons = fasta.iter().filter(|(k, _)| *k != id).map(|(_, v)| v.as_ref()).collect::<Vec<&str>>();

    let rna = problem(gene, exons);
    let prot = CodonTable::default().translate(&rna).unwrap();
    assert_eq!(&prot, "MVYIADKQHVASREAYGHMFKVCA");
}