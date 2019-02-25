use std::fs;

fn matches(s: &str, sub: &str) -> Vec<usize> {
    let mut v = Vec::new();
    for i in 0..s.len() - sub.len() {
        if sub == &s[i..i + sub.len()] {
            v.push(i + 1);
        }
    }
    v
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("rosalind_subs.txt")?;
    let lines = input.lines().collect::<Vec<&str>>();
    let output = matches(&lines[0].trim(), &lines[1].trim());
    for m in output {
        print!("{} ", m);
    }
    Ok(())
}

#[test]
fn sample() {
    let input = "GATATATGCATATACTT";
    let pat = "ATAT";
    let output = vec![2, 4, 10];
    assert_eq!(output, matches(input, pat));
}
