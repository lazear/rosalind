fn factorial(num: u64) -> u128 {
  match num {
    0 => 1,
    1 => 1,
    _ => factorial(num - 1) * num as u128,
  }
}

fn binomial(n: u64, r: u64) -> f64 {
  return (factorial(n) / factorial(r) / factorial(n - r)) as f64;
}

fn prob(k: u64, n: u64) -> f64 {
  let target_prob: f64 = 0.25; // probabilty of AaBb
  let mut total_prob = 0.;
  for i in 0..n {
    let ni = (2u64.pow(k as u32) - i) as i32;
    total_prob += binomial(2u64.pow(k as u32), i) as f64
      * target_prob.powi(i as i32)
      * (1f64 - target_prob).powi(ni);
  }
  1. - total_prob
}

fn main() {
  println!("{}", prob(2, 1));
}

#[test]
fn sample() {}
