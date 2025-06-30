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
        let mut a: Vec<usize> = (0..n).map(|_| cin.next()).collect();
        let mut b: Vec<usize> = (0..n).map(|_| cin.next()).collect();

        let mut k: usize = 0;
        let mut ans: Vec<(usize, usize)> = vec![];
        
        for i in 0..n {
            for j in 0..(n - 1 - i) {
                if a[j] > a[j + 1] {
                    a.swap(j, j + 1);
                    k += 1;
                    ans.push((1, j + 1));
                }
            }
        }
        for i in 0..n {
            for j in 0..(n - 1 - i) {
                if b[j] > b[j + 1] {
                    b.swap(j, j + 1);
                    k += 1;
                    ans.push((2, j + 1));
                }
            }
        }
        for i in 0..n {
            if a[i] > b[i] {
                k += 1;
                ans.push((3, i + 1));
            }
        }

        println!("{}", k);
        for (c, k) in ans {
            println!("{} {}", c, k);
        }

    }
}
