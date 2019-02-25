use std::fs;
use std::iter::Peekable;
use std::str::Chars;
use std::char;

struct Parser<'s> {
    input: Peekable<Chars<'s>>,
}

impl<'s> Parser<'s> {
    fn read_ident(input: &mut Peekable<Chars<'s>>,) -> String {
        input.take_while(|&ch| ch != '\n').filter(|&ch| !char::is_whitespace(ch)).collect()
    }

    fn read_sequence(input: &mut Peekable<Chars<'s>>,) -> String {
        input.take_while(|&ch| ch != '>').filter(|&ch| !char::is_whitespace(ch)).collect()
    }

    fn read(mut self) -> Vec<(String, String)> {
        let mut v = Vec::new(); 
        while let Some(_) = self.input.peek() {
            let id = Self::read_ident(&mut self.input);
            let seq = Self::read_sequence(&mut self.input);
            v.push((id, seq));
        }
        v
    }
}

fn gc_content(seq: &str) -> f32 {
    let l = seq.len() as f32;
    100.0 * seq.chars().filter(|&c| c == 'C' || c == 'G').count() as f32 / l
}

fn gc(input: &str) -> (String, f32) {
    let p = Parser { input: input.chars().peekable() }.read();
    let mut v = p.into_iter().map(|e| (e.0, gc_content(&e.1))).collect::<Vec<(String, f32)>>();
    v.sort_by(|a, b|a.1.partial_cmp(&b.1).unwrap());
    v.pop().unwrap()
}


fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("rosalind_gc.txt")?;
    let output = gc(&input);
    println!("{:?}", output);
    Ok(())
}

#[test]
fn sample() {
    let input = fs::read_to_string("sample.txt").unwrap();
    let output = gc(&input);
    assert_eq!(output.0, "Rosalind_0808");
    assert_eq!(output.1,  60.919540);
}
