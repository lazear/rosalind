fn problem(s: &str, array: &[f64]) -> Vec<f64> {
    let mut prob = vec![];
    for a in array {
        let gc = *a;
        let mut p = 1.0;
        for c in s.chars() {
            p *= match c {
                'A' | 'T' => (1.0 - gc) * 0.5,
                'C' | 'G' => gc * 0.5,
                _ => unimplemented!(),
            };
        }
        prob.push(p.log10());
    }
    prob
}

fn main() {
    let input = std::fs::read_to_string("rosalind_prob.txt").unwrap();
    let mut lines = input.lines();
    let s = lines.next().unwrap();
    let array = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|w| w.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    let res = problem(s, &array);
    for r in res {
        print!("{:.3}\t", r);
    }
}

#[test]
fn sample() {
    let s = "ACGATACAA";
    let array = [0.129, 0.287, 0.423, 0.476, 0.641, 0.742, 0.783];
    let expected = [-5.737, -5.217, -5.263, -5.360, -5.958, -6.628, -7.009];
    assert!(
        problem(s, &array)
            .iter()
            .zip(&expected)
            .fold(0.0, |acc, (a, b)| acc + (a - b).abs())
            < 0.005
    );
}
