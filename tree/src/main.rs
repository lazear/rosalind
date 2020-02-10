fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("rosalind_tree.txt").unwrap();
    let mut lines = input.lines();
    let n = lines.next().unwrap().trim().parse::<usize>().unwrap();
    let mut edge_count = 0;
    while let Some(_) = lines.next() {
        edge_count += 1;
    }
    // min edge count to complete a tree is n -1
    println!("{}", n - edge_count - 1);
    Ok(())
}
