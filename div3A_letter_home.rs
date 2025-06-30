use crate::helpers::*;
use std::io::{Read, Write};

fn solve<'a>(cin: &mut impl Iterator<Item = &'a str>, cout: &mut impl Write) -> Option<()> {
    let n: usize = get(cin)?;
    let s: usize = get(cin)?;

    let a: Vec<usize> = get_vec(cin, n)?;
    let x1 = a[0];
    let xn = a[n-1];
    drop(a);

    let xmid = x1 + (xn - x1) / 2;
    dbg!(xmid);

    let ans;

    if s < x1 {
        ans = xn - s;
    } else if s <= xmid {
        ans = 2 * (s - x1) + xn - s;
    } else if s < xn {
        ans = 2 * (xn - s) + s - x1;
    } else {
        ans = s - x1;
    }

    set(cout, ans);
    nl(cout);

    Some(())
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
