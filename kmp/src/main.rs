use utils::Fasta;

struct Searcher<'s> {
    table: Vec<usize>,
    bytes: &'s [u8],
}

struct Matches<'s, 'b> {
    searcher: &'s Searcher<'s>,
    text: &'b [u8],
    pos: usize,
}

impl<'s> Searcher<'s> {
    /// Construct a K-M-P backtracking failure table, except with the use of
    /// zeroes instead of -1
    ///
    /// This table finds potential backtracking locations within the search
    /// string
    ///
    /// https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm
    /// https://web.stanford.edu/class/cs97si/10-string-algorithms.pdf
    ///
    fn construct(word: &str) -> Searcher {
        let bytes = word.as_bytes();
        let mut table = Vec::with_capacity(bytes.len());

        table.push(0);

        for i in 1..bytes.len() {
            let mut longest = table[i - 1];
            // Recursively apply the failure function to the previous substring
            // to find the longest prefix
            while longest > 0 && bytes[i] != bytes[longest] {
                longest = table[longest - 1];
            }
            if bytes[i] == bytes[longest] {
                longest += 1;
            }
            table.push(longest)
        }

        Searcher { table, bytes }
    }

    fn matches<'a, 'b>(&'a self, text: &'b str) -> Matches<'a, 'b> {
        Matches {
            searcher: self,
            text: text.as_bytes(),
            pos: 0,
        }
    }
}

impl<'s, 'b> Iterator for Matches<'s, 'b> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let mut matched = 0;
        for i in self.pos..self.text.len() {
            while matched > 0 && self.text[i] != self.searcher.bytes[matched] {
                // Go to backtrack point
                matched = self.searcher.table[matched - 1];
            }

            if self.text[i] == self.searcher.bytes[matched] {
                // Increment prefix length
                matched += 1;
                // We hit a full match
                if matched == self.searcher.bytes.len() {
                    self.pos = i;
                    return Some(i - (matched - 1));
                }
            }
        }
        None
    }
}

fn main() -> std::io::Result<()> {
    let data = Fasta::parse_file("rosalind_kmp.txt")?;
    let seq = data.values().next().unwrap();
    let table = Searcher::construct(seq).table;
    println!(
        "{}",
        table
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    Ok(())
}

#[test]
fn sample() {
    let input = ">Rosalind_87
CAGCATGGTATCACAGCAGAG";
    let data = Fasta::parse_string(input);
    let seq = data.values().next().unwrap();
    let expected = vec![
        0, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 1, 2, 1, 2, 3, 4, 5, 3, 0, 0,
    ];
    assert_eq!(Searcher::construct(seq).table, expected);
}
