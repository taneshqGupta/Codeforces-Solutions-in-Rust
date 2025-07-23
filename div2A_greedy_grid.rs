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
        let [n, m] = std::array::from_fn(|_| cin.next::<usize>());
        if (n >= 2 && m >= 3) || (n >= 3 && m >= 2) {
            println!("{}", "YES");
        } else {
            println!("{}", "NO");
        }
    }
}
