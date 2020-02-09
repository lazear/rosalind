use std::collections::HashMap;

fn rabbits(
  n: usize,
  k: usize,
  d: usize,
  saved: &mut HashMap<usize, (usize, usize)>,
) -> (usize, usize) {
  // tuple holds the population and new bunnies
  if n == 1 {
    return (1, 1);
  } else if n == 2 {
    return (1, 0);
  }

  let a = match saved.get(&(n - 1)) {
    Some(a) => *a,
    None => rabbits(n - 1, k, d, saved),
  };

  let b = match saved.get(&(n - 2)) {
    Some(b) => *b,
    None => rabbits(n - 2, k, d, saved),
  };

  let mut c = (0, 0);
  if n > d {
    c = match saved.get(&(n - d)) {
      Some(b) => *b,
      None => rabbits(n - d, k, d, saved),
    };
  }

  saved.insert(n, (a.0 + b.0 * k - c.1, b.0 * k));
  (a.0 + b.0 * k - c.1, b.0 * k)
}

fn main() {
  let mut init = HashMap::new();
  let n = rabbits(5, 1, 3, &mut init);
  println!("{}", n.0);
}

#[test]
fn sample() {
  let mut init = HashMap::new();
  assert_eq!(rabbits(5, 1, 3, &mut init).0, 4);
}
