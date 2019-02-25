use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use utils::Fasta;

#[derive(Debug)]
pub struct Node<'s> {
    ids: Vec<&'s str>,
    matches: Vec<&'s str>,
}

#[derive(Debug)]
pub struct Graph<'s> {
    suffixes: HashMap<&'s str, Node<'s>>,
}

impl<'s> Graph<'s> {
    /// Construct an overlap graph with a matching length of N
    pub fn build(n: usize, data: &'s HashMap<String, String>) -> Graph<'s> {
        let mut suffixes = HashMap::new();
        // Build suffixes
        for (id, seq) in data {
            let l = seq.len();
            let start = l.saturating_sub(n);
            let entry = suffixes.entry(&seq[start..]).or_insert(Node {
                ids: Vec::new(),
                matches: Vec::new(),
            });
            entry.ids.push(id);
        }
        // Match prefixes
        for (id, seq) in data {
            let pref = &seq[0..n];
            if let Some(node) = suffixes.get_mut(pref) {
                node.matches.push(id);
            }
        }
        Graph { suffixes }
    }

    /// Return a list of all non-cylic edges
    pub fn edges(&self) -> HashSet<(&str, &str)> {
        let mut v = HashSet::new();
        for (i, seq) in &self.suffixes {
            for &id in &seq.ids {
                for &m in &seq.matches {
                    if id != m {
                        v.insert((id, m));
                    }
                }
            }
        }
        v
    }
}

fn main() -> std::io::Result<()> {
    let map = Fasta::parse_file("rosalind_grph.txt")?;
    let graph = Graph::build(3, &map);
    let mut f = File::create("out.txt")?;
    for edge in graph.edges() {
        writeln!(f, "{} {}", edge.0, edge.1)?;
    }
    Ok(())
}

#[test]
fn sample() {
    let map = Fasta::parse_file("sample.txt").unwrap();
    let edges = [
        ("Rosalind_2391", "Rosalind_2323"),
        ("Rosalind_0498", "Rosalind_0442"),
        ("Rosalind_0498", "Rosalind_2391"),
    ]
    .into_iter()
    .collect::<HashSet<_>>();

    let graph = Graph::build(3, &map);

    assert_eq!(edges, graph.edges().iter().collect());
}
