//! For this problem, we construct an extremely simple recursively backtracking
//! finite automaton to identify protein motifs. We could just use regex...
//! but where's the fun in that?
use reqwest;
use utils::Fasta;

#[derive(Copy, Clone, Debug)]
enum Pat {
    Char(char),
    Not(char),
    Branch(char, char),
}

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
enum State {
    Start,
    Finish,
    Transition,
}

struct Automaton {
    pat: Vec<Pat>,
    cur: usize,
}

#[derive(Debug)]
struct Match<'s> {
    slice: &'s str,
    start: usize,
    end: usize,
}

struct Matches<'s> {
    s: &'s str,
    start_idx: usize,
    backtrack: u8,
    aut: Automaton,
}

impl<'s> Iterator for Matches<'s> {
    type Item = Match<'s>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut t = None;
        self.aut.cur = 0;
        for (idx, ch) in self.s[self.start_idx..].char_indices() {
            match self.aut.next_state(ch) {
                State::Start => {
                    if t.is_some() && self.backtrack < 2 {
                        // backtrack ... very inefficient
                        self.backtrack += 1;
                        self.start_idx += 1;
                        return self.next();
                    } else {
                        t = None;
                        self.backtrack = 0;
                    }
                }
                State::Transition => {
                    if t.is_none() {
                        t = Some(idx)
                    }
                }
                State::Finish => {
                    let start = t.unwrap_or(0) + self.start_idx;
                    let end = idx + self.start_idx + 1;
                    let slice = &self.s[start..end];
                    // Start searching at the next character after the beginning
                    // of the match we're returning
                    self.start_idx = start + 1;
                    self.aut.cur = 0;
                    self.backtrack = 0;
                    return Some(Match { slice, start, end });
                }
            }
        }
        None
    }
}

impl Automaton {
    fn construct(pat: Vec<Pat>) -> Automaton {
        Automaton { pat, cur: 0 }
    }

    fn next_state(&mut self, ch: char) -> State {
        let start = self.cur;
        match self.pat[start] {
            Pat::Char(c) => {
                if c == ch {
                    self.cur += 1;
                }
            }
            Pat::Not(c) => {
                if c != ch {
                    self.cur += 1;
                }
            }
            Pat::Branch(c1, c2) => {
                if ch == c1 || ch == c2 {
                    self.cur += 1;
                }
            }
        }
        if self.cur == 0 || self.cur == start {
            self.cur = 0;
            State::Start
        } else if self.cur == self.pat.len() {
            State::Finish
        } else {
            State::Transition
        }
    }

    fn matches<'s>(self, s: &'s str) -> Matches<'s> {
        Matches {
            s,
            start_idx: 0,
            backtrack: 0,
            aut: self,
        }
    }
}

fn problem(s: &str) -> Vec<usize> {
    // N{P}[ST]{P}
    let pat = vec![
        Pat::Char('N'),
        Pat::Not('P'),
        Pat::Branch('S', 'T'),
        Pat::Not('P'),
    ];

    Automaton::construct(pat)
        .matches(s)
        .map(|m| m.start + 1)
        .collect::<Vec<usize>>()
}

fn main() -> std::io::Result<()> {
    let s = std::fs::read_to_string("rosalind_mprt.txt")?;
    let ids = s.lines().collect::<Vec<_>>();
    for id in ids {
        let body = reqwest::get(&format!("http://www.uniprot.org/uniprot/{}.fasta", id))
            .unwrap()
            .text()
            .unwrap();
        if body == "" {
            continue;
        }
        let input = Fasta::parse_string(&body);

        let seq = input.values().next().unwrap();
        let res = problem(&seq)
            .into_iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        if res.is_empty() {
            continue;
        }

        println!("{}\n{}", id, res);
    }
    Ok(())
}

#[test]
fn sample() {
    let s = std::fs::read_to_string("sample.txt").unwrap();
    let ids = s.lines().collect::<Vec<_>>();
    let res = vec![
        vec![85, 118, 142, 306, 395],
        vec![47, 115, 116, 382, 409],
        vec![79, 109, 135, 248, 306, 348, 364, 402, 485, 501, 614],
    ];

    for (id, result) in ids.into_iter().zip(res.into_iter()) {
        let body = reqwest::get(&format!("http://www.uniprot.org/uniprot/{}.fasta", id))
            .unwrap()
            .text()
            .unwrap();

        let input = Fasta::parse_string(&body);
        let seq = input.values().next().unwrap();

        assert_eq!(problem(&seq), result);
    }
}
