use std::fs::File;
use std::io::Write;
use utils::Fasta;

/// Transpose a matrix of An,m into a matrix of Am,n
fn transpose<T, I, V>(v: V) -> Vec<Vec<T>>
where
    T: Copy,
    I: AsRef<[T]>,
    V: AsRef<[I]>,
{
    let len = v.as_ref()[0].as_ref().len();
    let mut outer = Vec::with_capacity(len);
    for j in 0..len {
        outer.push(v.as_ref().iter().map(|x| x.as_ref()[j]).collect());
    }
    outer
}

fn index(ch: char) -> usize {
    match ch {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        'T' => 3,
        _ => panic!("invalid nucleotide {}", ch),
    }
}

fn matrix(input: Vec<&str>) -> (String, Vec<Vec<usize>>) {
    let revidx = ['A', 'C', 'G', 'T'];
    let m = input
        .into_iter()
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let t = transpose(m);

    let mut consensus = String::new();
    let mut outer = Vec::new();
    for v in t {
        let mut array = vec![0, 0, 0, 0];
        for nt in v {
            array[index(nt)] += 1;
        }

        let max = array.iter().max().unwrap();
        let idx = array.iter().position(|e| e == max).unwrap();
        outer.push(array);
        consensus.push(revidx[idx]);
    }
    (consensus, transpose(outer))
}

fn main() -> std::io::Result<()> {
    let input = Fasta::parse_file("rosalind_cons.txt")?;
    let (consensus, out) = matrix(input.values().map(|s| s.trim()).collect());
    let revidx = ['A', 'C', 'G', 'T'];

    let mut f = File::create("out.txt")?;
    writeln!(f, "{}", consensus)?;
    for (nt, v) in revidx.iter().zip(out.iter()) {
        writeln!(
            f,
            "{}: {}",
            nt,
            v.iter()
                .map(|el| format!("{} ", el))
                .collect::<Vec<String>>()
                .join(" ")
        )?;
    }
    Ok(())
}
