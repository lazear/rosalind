fn expected_val(populations: Vec<usize>, probs: Vec<f32>, offspring: usize) -> f32 {
  let c = probs
    .iter()
    .zip(populations.iter())
    .map(|(c1, c2)| (c1 * *c2 as f32) * offspring as f32)
    .sum();
  c
}

fn main() {
  let probs = vec![1., 1., 1., 0.75, 0.5, 0.]; // A phenotype probabilites
  let populations = vec![1, 0, 0, 1, 0, 1];
  println!("{}", expected_val(populations, probs, 2));
}

#[test]
fn sample() {
  assert_eq!(
    expected_val(vec![1, 0, 0, 1, 0, 1], vec![1., 1., 1., 0.75, 0.5, 0.], 2),
    3.5
  )
}
