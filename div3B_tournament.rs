use std::array;

#[derive(Default, Debug)]
struct Scanner(Vec<String>);
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(c) = self.0.pop() {
                return c.parse().ok().unwrap();
            }
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            self.0 = s.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut cin = Scanner::default();
    let t = cin.next();
    for _ in 0..t {
        let [n, j, k] = array::from_fn(|_| cin.next::<usize>());
        let a: Vec<usize> = (0..n).map(|_| cin.next()).collect();

        let mx = *a.iter().max().unwrap();

        let ans;

        if k == 1 && a[j - 1] == mx {
            ans = "yes";
        } else if k == 1 {
            ans = "no";
        } else {
            ans = "yes";
        }

        print!("{}\n", ans);
    }
}
