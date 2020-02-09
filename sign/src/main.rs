use std::fs::{self};
use std::io::Write;

fn signed_perm(
    start: usize,
    n: usize,
    slice: &mut [i32],
    input_set: &Vec<i32>,
    out: &mut Vec<Vec<i32>>,
) {
    if start == n {
        out.push(Vec::from(slice));
        return;
    }
    for i in 0..input_set.len() {
        let mut bypass = false;
        for j in 0..start {
            if slice[j] == input_set[i] * -1 || slice[j] == input_set[i] {
                bypass = true;
                break;
            }
        }
        if !bypass {
            slice[start] = input_set[i];
            signed_perm(start + 1, n, slice, input_set, out);
        }
    }
}
fn main() -> std::io::Result<()> {
    let mut out = Vec::<Vec<i32>>::new();
    let input = fs::read_to_string("rosalind_sign.txt")?
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut input_set = Vec::<i32>::new();

    (1..=input as i32).for_each(|v| {
        input_set.push(v);
        input_set.push(v * -1);
    });
    let mut slice = vec![0; input];
    println!("{:?}", &slice);
    signed_perm(0, input, &mut slice, &input_set, &mut out);
    println!("{:?}", out);

    let mut f = fs::File::create("out.txt")?;
    out.into_iter().for_each(|v| {
        writeln!(
            f,
            "{}",
            v.into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
        .unwrap()
    });
    Ok(())
}

#[test]
fn sample() {
    let out2 = vec![
        [1, 2].to_vec(),
        [1, -2].to_vec(),
        [-1, 2].to_vec(),
        [-1, -2].to_vec(),
        [2, 1].to_vec(),
        [2, -1].to_vec(),
        [-2, 1].to_vec(),
        [-2, -1].to_vec(),
    ]
    .to_vec();
    let mut input_set = Vec::<i32>::new();
    (1..=2 as i32).for_each(|v| {
        input_set.push(v);
        input_set.push(v * -1);
    });
    let mut slice = vec![0; 2];
    let mut out = Vec::<Vec<i32>>::new();

    signed_perm(0, 2, &mut slice, &input_set, &mut out);
    assert_eq!(out2, out);
}
