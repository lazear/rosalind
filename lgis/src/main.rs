use std::fs::{self, File};
use std::io::Write;

fn lgis(seq: &Vec<usize>) -> Vec<usize> {
    let mut res = Vec::<usize>::new();
    for i in 0..seq.len() {
        let mut tmp_res = Vec::<usize>::new();
        tmp_res.push(seq[i]);
        for j in i + 1..seq.len() {
            if tmp_res.last().unwrap() < &seq[j] {
                tmp_res.push(seq[j]);
            } else if tmp_res.last().unwrap() > &seq[j] {
                if tmp_res.len() > res.len() {
                    res = tmp_res.clone();
                }
                while tmp_res.len() > 0 && tmp_res.last().unwrap() >= &seq[j] {
                    tmp_res.pop();
                }
                tmp_res.push(seq[j]);
            }
        }
        if tmp_res.len() > res.len() {
            res = tmp_res.clone();
        }
    }
    res
}

// same function above for decreasing
fn lgis_d(seq: &Vec<usize>) -> Vec<usize> {
    let mut res = Vec::<usize>::new();
    for i in 0..seq.len() {
        let mut tmp_res = Vec::<usize>::new();
        tmp_res.push(seq[i]);
        for j in i + 1..seq.len() {
            if tmp_res.last().unwrap() > &seq[j] {
                tmp_res.push(seq[j]);
            } else if tmp_res.last().unwrap() < &seq[j] {
                if tmp_res.len() > res.len() {
                    res = tmp_res.clone();
                }

                while tmp_res.len() > 0 && tmp_res.last().unwrap() <= &seq[j] {
                    tmp_res.pop();
                }
                tmp_res.push(seq[j]);
            }
        }
        if tmp_res.len() > res.len() {
            res = tmp_res.clone();
        }
    }
    res
}

fn main() -> std::io::Result<()> {
    let s = fs::read_to_string("rosalind_lgis.txt")?;
    //let seq_len = s.lines().nth(0).unwrap().trim().parse::<usize>().unwrap();
    let seq = s
        .lines()
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let inc_out = lgis(&seq);
    let dec_out = lgis_d(&seq);

    let mut f = File::create("out.txt")?;

    writeln!(
        f,
        "{}",
        inc_out
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
    .unwrap();

    writeln!(
        f,
        "{}",
        dec_out
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
    .unwrap();

    Ok(())
}

#[test]
fn sample() {}
