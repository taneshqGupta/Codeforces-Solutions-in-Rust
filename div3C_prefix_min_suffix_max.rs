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
        let a: Vec<u32> = (0..n).map(|_| cin.next()).collect();

        let mut ans: Vec<bool> = vec![false; n];
        let mut no_less_before: Vec<bool> = vec![true; n];
        let mut no_more_after: Vec<bool> = vec![true; n];

        // let maxm = *a.iter().max().unwrap();
        // let minm = *a.iter().min().unwrap();

        let mut curr_min: u32 = 2e6 as u32;
        for i in 0..n {
            if a[i] < curr_min {
                curr_min = a[i];
            }
            if a[i] > curr_min {
                no_less_before[i] = false;
            }
        }

        let mut curr_max = 0;
        for i in (0..n).rev() {
            if a[i] > curr_max {
                curr_max = a[i];
            } else {
                no_more_after[i] = false;
            }
        }

        for i in 0..n {
            if no_less_before[i] || no_more_after[i] {
                ans[i] = true;
            } else {
                ans[i] = false;
            }
        }

        for c in ans {
            print!("{}", if c { 1 } else { 0 });
        }
        println!();
    }
}
