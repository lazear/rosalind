use std::collections::HashMap;

fn rabbits(n: usize, k: usize, saved: &mut HashMap<usize, usize>) -> usize {
    if n <= 2 {
        return 1;
    }
    let a = match saved.get(&(n - 1)) {
        Some(a) => *a,
        None => rabbits(n - 1, k, saved),
    };
    let b = match saved.get(&(n - 2)) {
        Some(b) => *b,
        None => rabbits(n - 2, k, saved),
    };
    saved.insert(n, a + b * k);
    a + b * k
}

fn main() {
    let mut init = HashMap::new();

    let n = rabbits(29, 4, &mut init);
    println!("{}", n);
}

#[test]
fn sample() {
    let mut init = HashMap::new();
    assert_eq!(rabbits(5, 3, &mut init), 19);
}
