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
        let m: usize = cin.next();
        if (n >= 2 && m >= 3) || (n >= 3 && m >= 2) {
            println!("{}", "YES");
        } else {
            println!("{}", "NO");
        }
    }
}