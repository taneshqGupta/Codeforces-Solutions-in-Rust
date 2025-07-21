#[derive(Default)]
struct Scanner(Vec<String>);
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.0.pop() {
                return token.parse().ok().unwrap();
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.0 = input.split_whitespace().rev().map(String::from).collect();
        }
    }
    fn get<T: std::str::FromStr, const N: usize>(&mut self) -> [T; N] {
        std::array::from_fn(|_| self.next())
    }
}

fn main() {
    let mut cin = Scanner::default();
    let t: usize = cin.next();
    for _ in 0..t {
        let n: usize = cin.next();
        let mut ans: usize = 0;
        for _ in 0..n {
            let [a, b, c, d]: [usize; 4] = cin.get();
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
        print!("{}\n", ans);
    }
}
