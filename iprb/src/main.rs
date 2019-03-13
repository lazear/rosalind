fn main() {
    let k = 17.0; // homozygous dominant
    let m = 29.0; // hetero
    let n = 26.0; // homozygous recess
    let sum = k + m + n;

    let prb = 1.
        - (n / sum) * ((n - 1.0) / (sum - 1.0))
        - (n / sum) * (m / (sum - 1.))
        - (m / sum) * ((m - 1.) / (sum - 1.)) * 0.25;

    println!("{}", prb);
}
