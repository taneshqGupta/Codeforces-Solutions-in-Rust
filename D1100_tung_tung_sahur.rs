use crate::helpers::*;
use std::io::{Read, Write};

fn solve<'a>(cin: &mut impl Iterator<Item = &'a str>, cout: &mut impl Write) -> Option<()> {
    let p: String = get(cin)?;
    let s: String = get(cin)?;
    
    fn memo(input: &str) -> Vec<(char, u32)> {
        let mut memx = Vec::new();
        let mut chars = input.chars();
        
        let mut curr = chars.next().unwrap();
        let mut count = 1;
        
        for c in chars {
            if c == curr {
                count += 1;
            }
            else {
                memx.push((curr, count));
                curr = c;
                count = 1;
            }
        }
        memx.push((curr, count));
        return memx;
    }
    
    let p = memo(&p);
    let s = memo(&s);
    
    if p.len() != s.len() {
        set(cout, "NO"); nl(cout);
        return Some(());
    }
    
    let mut ys = true;
    let (mut i, mut j) = (0, 0);

    while i < p.len() && j < s.len() {
        if s[j].0 != p[i].0 {
            ys = false;
            break;
        }
        if s[j].1 < p[i].1 || s[j].1 > 2 * p[i].1 {
            ys = false;
            break;
        }
        i += 1; j += 1;
    }
    // dbg!(&i, &p, &j, &s);
    if i != p.len() || j != s.len() {
        ys = false;
    }  

    drop(p);
    drop(s);

    if ys {
        set(cout, "YES"); nl(cout);
    }
    else {
        set(cout, "NO"); nl(cout);
    }

    return Some(());
}

fn main() {
    let mut cout = std::io::BufWriter::new(std::io::stdout());
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut cin = s.split_ascii_whitespace();

    // -- if there are test cases --
    let _t: usize = get(&mut cin).unwrap();
    // -- if there are test cases --

    while let Some(_) = solve(&mut cin, &mut cout) {}

}

#[allow(dead_code)]
mod helpers {
    use std::{fmt::Display, io::Write, str::FromStr};

    pub fn set<T: Display>(cout: &mut impl Write, a: T) {
        write!(cout, "{}", a).ok();
    }

    pub fn get<'a, T: FromStr>(cin: &mut impl Iterator<Item = &'a str>) -> Option<T> {
        Some(cin.next()?.parse::<T>().ok()?)
    }

    pub fn get_vec<'a, T: FromStr>(
        cin: &mut impl Iterator<Item = &'a str>,
        n: usize,
    ) -> Option<Vec<T>> {
        Some(cin.take(n).filter_map(|s| s.parse::<T>().ok()).collect())
    }

    pub fn set_vec<T: Display>(cout: &mut impl Write, aa: &[T]) {
        let mut iter = aa.iter().peekable();
        while let Some(item) = iter.next() {
            write!(cout, "{}", item).ok();
            if iter.peek().is_some() {
                write!(cout, " ").ok();
            }
        }
    }

    pub fn nl(cout: &mut impl Write) {
        writeln!(cout, "").ok();
    }

    pub fn sp(cout: &mut impl Write) {
        write!(cout, " ").ok();
    }
}

