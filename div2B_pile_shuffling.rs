struct Scanner(Vec<String>);
impl Scanner {
    fn new() -> Self {
        let input = std::io::read_to_string(std::io::stdin()).unwrap();
        Scanner(input.split_whitespace().map(String::from).rev().collect())
    }
    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.0.pop().unwrap().parse().ok().unwrap()
    }
}

fn main() {
    let mut cin = Scanner::new();
    let t: usize = cin.next();
    for _ in 0..t {
        let n: usize = cin.next();
        let mut ans: usize = 0;
        for _ in 0..n {
            let a: usize = cin.next();
            let b: usize = cin.next();
            let c: usize = cin.next();
            let d: usize = cin.next();
            if a > c && b > d {
                ans += a - c;
                ans += b - d;
                ans += c;
            }
            else if a > c {
                ans += a - c;
            }
            else if b > d {
                ans += a;
                ans += b - d;
            }
            else {
                continue;
            }
        }
        println!("{}", ans);
    }
}