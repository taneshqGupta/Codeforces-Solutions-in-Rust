use crate::helpers::*;
use std::io::{Read, Write};

fn solve<'a>(cin: &mut impl Iterator<Item = &'a str>, cout: &mut impl Write) -> Option<()> {
    let n: usize = get(cin)?;
    let q: usize = get(cin)?;
    let a: Vec<usize> = get_vec(cin, n)?;

    let mut oddroll: Vec<usize> = Vec::new();
    if a[0] % 2 == 1 {
        oddroll.push(1);
    } else {
        oddroll.push(0);
    }

    for i in 1..a.len() {
        if a[i] % 2 != 0 {
            oddroll.push(oddroll[i - 1] + 1);
        } else {
            oddroll.push(oddroll[i - 1]);
        }
    }
    // dbg!(&oddroll);
    for _ in 0..q {
        let l: usize = get(cin)?;
        let r: usize = get(cin)?;
        let k: usize = get(cin)?;
        let k_odds;
        if k % 2 == 1 {
            k_odds = r - l + 1;
        } else {
            k_odds = 0;
        }
        let odds_gone;
        if a[l - 1] % 2 == 1 {
            odds_gone = oddroll[r - 1] - oddroll[l - 1] + 1;
        } else {
            odds_gone = oddroll[r - 1] - oddroll[l - 1];
        }
        // dbg!(&l, &r, &k, &k_odds, &odds_gone);
        let new_odds = oddroll[n-1] - odds_gone + k_odds;
        
        if new_odds % 2 == 1 {
            set(cout, "yes");
            nl(cout);
        } else {
            set(cout, "no");
            nl(cout);
        }
    }

    return Some(());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cout = std::io::BufWriter::new(std::io::stdout());
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    let mut cin = s.split_ascii_whitespace();

    // -- if there are test cases --
    let _t: usize = get(&mut cin).ok_or("comment this line out if there are no test cases")?;
    // -- if there are test cases --

    while let Some(_) = solve(&mut cin, &mut cout) {}

    Ok(())
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
