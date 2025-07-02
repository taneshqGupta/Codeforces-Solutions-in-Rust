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
        let (n, j, k): (usize, usize, usize) = (cin.next(), cin.next(), cin.next());
        let a: Vec<usize> = (0..n).map(|_| cin.next()).collect();

        let mx = *a.iter().max().unwrap();

        let ans;

        if k == 1 && a[j-1] == mx {
            ans = "yes";
        }

        else if k == 1 {
            ans = "no";
        }

        else {
            ans = "yes";
        }

        println!("{}", ans);
    }
}
