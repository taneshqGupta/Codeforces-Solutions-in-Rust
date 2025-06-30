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
        let (n, m): (usize, usize) = (cin.next(), cin.next());
        let a: Vec<Vec<usize>> = (0..n)
            .map(|_| (0..m).map(|_| cin.next()).collect())
            .collect();

        let mx = *a.iter().flatten().max().unwrap();
        let max_nums: usize = a
            .iter()
            .flatten()
            .map(|&x| if x == mx { 1 } else { 0 })
            .sum();

        let mut row_sums = vec![0; n];
        let mut col_sums = vec![0; m];

        for i in 0..n {
            for j in 0..m {
                if a[i][j] == mx {
                    row_sums[i] += 1;
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if a[i][j] == mx {
                    col_sums[j] += 1;
                }
            }
        }

        // dbg!(&a, &max_nums, &max_tally, &row_sums, &col_sums);
        let mut max_decreases = false;

        'outer: for i in 0..n {
            for j in 0..m {
                let sumx = row_sums[i] + col_sums[j] - if a[i][j] == mx { 1 } else { 0 };
                if sumx == max_nums {
                    max_decreases = true;
                    break 'outer;
                }
            }
        }

        drop(row_sums);
        drop(col_sums);
        drop(a);

        println!("{}", if max_decreases { mx - 1 } else { mx });
    }
}
