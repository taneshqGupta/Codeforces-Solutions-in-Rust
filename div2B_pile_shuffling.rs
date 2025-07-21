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
    let t: usize = cin.next();
    for _ in 0..t {
        let n: usize = cin.next();
        let mut ans = 0;
        for _ in 0..n {
            let [a, b, c, d] = std::array::from_fn(|_| cin.next::<usize>());
            if a > c && b > d {
                ans += a - c;
                ans += b - d;
                ans += c;
            } else if a > c {
                ans += a - c;
            } else if b > d {
                ans += a;
                ans += b - d;
            } else {
                continue;
            }
        }
        println!("{}", ans);
    }
}
