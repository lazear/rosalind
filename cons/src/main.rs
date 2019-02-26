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

fn matrix(input: Vec<&str>) -> Vec<Vec<usize>> {
    let revidx = ['A', 'C', 'G', 'T'];
    let m = input
        .into_iter()
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let t = transpose(m);
    println!("{}", t.len());

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
    println!("{}", consensus);
    transpose(outer)
}

fn main() -> std::io::Result<()> {
    let input = Fasta::parse_file("rosalind_cons.txt")?;
    input.iter().for_each(|(k, v)| println!("{}\n{}", k, v));
    let out = matrix(input.values().map(|s| s as &str).collect());
    let revidx = ['A', 'C', 'G', 'T'];
    for (nt, v) in revidx.iter().zip(out.iter()) {
        println!(
            "{}: {}",
            nt,
            v.iter()
                .map(|el| format!("{} ", el))
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
    Ok(())
}
