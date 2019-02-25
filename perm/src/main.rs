use std::fs::{self, File};
use std::io::Write;

fn heaps(n: usize, slice: &mut [usize], output: &mut Vec<Vec<usize>>) {
    if n == 1 {
        output.push(Vec::from(slice));
        return;
    }
    for i in 0..n {
        heaps(n - 1, slice, output);
        if n % 2 == 0 {
            slice.swap(i, n - 1);
        } else {
            slice.swap(0, n - 1);
        }
    }
    //heaps(n - 1, slice, output)
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("rosalind_perm.txt")?
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut slice = (1..=input).collect::<Vec<usize>>();
    let mut out = Vec::new();
    heaps(input, &mut slice, &mut out);
    let mut f = fs::File::create("out.txt")?;
    writeln!(f, "{}", out.len());
    out.into_iter().for_each(|v| {
        writeln!(
            f,
            "{}",
            v.into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
        .unwrap()
    });
    Ok(())
}

#[test]
fn sample() {
    let mut out = Vec::new();
    heaps(3, &mut [1, 2, 3], &mut out);
    assert_eq!(out.len(), 6);
    assert_eq!(
        out,
        vec![
            vec![1, 2, 3],
            vec![2, 1, 3],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 3, 1],
            vec![3, 2, 1]
        ]
    );
}
